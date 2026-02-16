use crate::Scope;
use crate::ast::{Expr, Result, Span};

use super::{Node, render_nodes};

#[derive(Debug, Clone, PartialEq)]
pub struct MatchNode {
    pub expr: Expr,
    pub cases: Vec<MatchCase>,
    pub default: Option<Vec<Node>>,
    pub span: Span,
}

impl MatchNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let expr_val = self.expr.eval(scope)?;

        for case in &self.cases {
            let case_val = case.value.eval(scope)?;
            if expr_val == case_val {
                return render_nodes(&case.body, scope);
            }
        }

        if let Some(default_body) = &self.default {
            return render_nodes(default_body, scope);
        }

        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub value: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{TextNode, ValueExpr};

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

    #[test]
    fn render_matching_case() {
        let scope = Scope::new();
        let node = MatchNode {
            expr: val_expr(xval::Value::from_str("b")),
            cases: vec![
                MatchCase {
                    value: val_expr(xval::Value::from_str("a")),
                    body: vec![text("A")],
                    span: Span::new(0, 1),
                },
                MatchCase {
                    value: val_expr(xval::Value::from_str("b")),
                    body: vec![text("B")],
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
                body: vec![text("A")],
                span: Span::new(0, 1),
            }],
            default: Some(vec![text("default")]),
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
                body: vec![text("A")],
                span: Span::new(0, 1),
            }],
            default: None,
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "");
    }
}
