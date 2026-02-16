use crate::Scope;
use crate::ast::{Expr, Result, Span};

use super::BlockNode;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchNode {
    pub expr: Expr,
    pub cases: Vec<MatchCase>,
    pub default: Option<BlockNode>,
    pub span: Span,
}

impl MatchNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let expr_val = self.expr.eval(scope)?;

        for case in &self.cases {
            let case_val = case.value.eval(scope)?;
            if expr_val == case_val {
                return case.body.render(scope);
            }
        }

        if let Some(default_body) = &self.default {
            return default_body.render(scope);
        }

        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub value: Expr,
    pub body: BlockNode,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Node, TextNode, ValueExpr};

    fn val_expr(v: xval::Value) -> Expr {
        Expr::Value(ValueExpr {
            value: v,
            span: Span::new(0, 1),
        })
    }

    fn text(s: &str) -> Node {
        Node::Text(TextNode {
            text: s.into(),
            span: Span::new(0, 1),
        })
    }

    fn block(nodes: Vec<Node>) -> BlockNode {
        BlockNode {
            nodes,
            span: Span::new(0, 1),
        }
    }

    #[test]
    fn render_matching_case() {
        let scope = Scope::new();
        let node = MatchNode {
            expr: val_expr(xval::Value::from_str("b")),
            cases: vec![
                MatchCase {
                    value: val_expr(xval::Value::from_str("a")),
                    body: block(vec![text("A")]),
                    span: Span::new(0, 1),
                },
                MatchCase {
                    value: val_expr(xval::Value::from_str("b")),
                    body: block(vec![text("B")]),
                    span: Span::new(0, 1),
                },
            ],
            default: None,
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "B");
    }

    #[test]
    fn render_default() {
        let scope = Scope::new();
        let node = MatchNode {
            expr: val_expr(xval::Value::from_str("c")),
            cases: vec![MatchCase {
                value: val_expr(xval::Value::from_str("a")),
                body: block(vec![text("A")]),
                span: Span::new(0, 1),
            }],
            default: Some(block(vec![text("default")])),
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "default");
    }

    #[test]
    fn render_no_match_no_default() {
        let scope = Scope::new();
        let node = MatchNode {
            expr: val_expr(xval::Value::from_str("c")),
            cases: vec![MatchCase {
                value: val_expr(xval::Value::from_str("a")),
                body: block(vec![text("A")]),
                span: Span::new(0, 1),
            }],
            default: None,
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "");
    }
}
