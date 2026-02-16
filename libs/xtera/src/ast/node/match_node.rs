use crate::Scope;
use crate::ast::{Expr, Result, Span};

use super::BlockNode;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchNode {
    pub expr: Expr,
    pub arms: Vec<MatchNodeArm>,
    pub default: Option<BlockNode>,
    pub span: Span,
}

impl MatchNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let expr_val = self.expr.eval(scope)?;

        for arm in &self.arms {
            let pattern_val = arm.pattern.eval(scope)?;
            if expr_val == pattern_val {
                return arm.body.render(scope);
            }
        }

        if let Some(default_body) = &self.default {
            return default_body.render(scope);
        }

        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchNodeArm {
    pub pattern: Expr,
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
    fn render_matching_arm() {
        let scope = Scope::new();
        let node = MatchNode {
            expr: val_expr(xval::Value::from_str("b")),
            arms: vec![
                MatchNodeArm {
                    pattern: val_expr(xval::Value::from_str("a")),
                    body: block(vec![text("A")]),
                    span: Span::new(0, 1),
                },
                MatchNodeArm {
                    pattern: val_expr(xval::Value::from_str("b")),
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
            arms: vec![MatchNodeArm {
                pattern: val_expr(xval::Value::from_str("a")),
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
            arms: vec![MatchNodeArm {
                pattern: val_expr(xval::Value::from_str("a")),
                body: block(vec![text("A")]),
                span: Span::new(0, 1),
            }],
            default: None,
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "");
    }
}
