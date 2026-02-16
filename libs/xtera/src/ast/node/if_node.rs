use crate::Scope;
use crate::ast::{Expr, Result, Span, is_truthy};

use super::{Node, render_nodes};

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub branches: Vec<IfBranch>,
    pub else_body: Option<Vec<Node>>,
    pub span: Span,
}

impl IfNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        for branch in &self.branches {
            let cond = branch.condition.eval(scope)?;

            if is_truthy(&cond) {
                return render_nodes(&branch.body, scope);
            }
        }

        if let Some(else_body) = &self.else_body {
            return render_nodes(else_body, scope);
        }

        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{TextNode, ValueExpr};

    fn val(v: xval::Value) -> Expr {
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
    fn render_true_branch() {
        let scope = Scope::new();
        let node = IfNode {
            branches: vec![IfBranch {
                condition: val(xval::Value::from_bool(true)),
                body: vec![text("yes")],
                span: Span::new(0, 1),
            }],
            else_body: None,
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "yes");
    }

    #[test]
    fn render_false_with_else() {
        let scope = Scope::new();
        let node = IfNode {
            branches: vec![IfBranch {
                condition: val(xval::Value::from_bool(false)),
                body: vec![text("yes")],
                span: Span::new(0, 1),
            }],
            else_body: Some(vec![text("no")]),
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "no");
    }

    #[test]
    fn render_no_match_no_else() {
        let scope = Scope::new();
        let node = IfNode {
            branches: vec![IfBranch {
                condition: val(xval::Value::from_bool(false)),
                body: vec![text("yes")],
                span: Span::new(0, 1),
            }],
            else_body: None,
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "");
    }

    #[test]
    fn render_else_if() {
        let scope = Scope::new();
        let node = IfNode {
            branches: vec![
                IfBranch {
                    condition: val(xval::Value::from_bool(false)),
                    body: vec![text("first")],
                    span: Span::new(0, 1),
                },
                IfBranch {
                    condition: val(xval::Value::from_bool(true)),
                    body: vec![text("second")],
                    span: Span::new(0, 1),
                },
            ],
            else_body: Some(vec![text("else")]),
            span: Span::new(0, 1),
        };
        
        assert_eq!(node.render(&scope).unwrap(), "second");
    }
}
