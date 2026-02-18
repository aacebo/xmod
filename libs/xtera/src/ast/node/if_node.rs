use crate::Scope;
use crate::ast::{Expr, Result, Span, is_truthy};

use super::BlockNode;

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub branches: Vec<IfBranch>,
    pub else_body: Option<BlockNode>,
    pub span: Span,
}

impl IfNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        for branch in &self.branches {
            let cond = branch.condition.eval(scope)?;

            if is_truthy(&cond) {
                return branch.body.render(scope);
            }
        }

        if let Some(else_body) = &self.else_body {
            return else_body.render(scope);
        }

        Ok(String::new())
    }
}

impl std::fmt::Display for IfNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: BlockNode,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Node, TextNode, ValueExpr};

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

    fn block(nodes: Vec<Node>) -> BlockNode {
        BlockNode {
            nodes,
            span: Span::new(0, 1),
        }
    }

    #[test]
    fn render_true_branch() {
        let scope = Scope::new();
        let node = IfNode {
            branches: vec![IfBranch {
                condition: val(xval::valueof!(true)),
                body: block(vec![text("yes")]),
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
                condition: val(xval::valueof!(false)),
                body: block(vec![text("yes")]),
                span: Span::new(0, 1),
            }],
            else_body: Some(block(vec![text("no")])),
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "no");
    }

    #[test]
    fn render_no_match_no_else() {
        let scope = Scope::new();
        let node = IfNode {
            branches: vec![IfBranch {
                condition: val(xval::valueof!(false)),
                body: block(vec![text("yes")]),
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
                    condition: val(xval::valueof!(false)),
                    body: block(vec![text("first")]),
                    span: Span::new(0, 1),
                },
                IfBranch {
                    condition: val(xval::valueof!(true)),
                    body: block(vec![text("second")]),
                    span: Span::new(0, 1),
                },
            ],
            else_body: Some(block(vec![text("else")])),
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "second");
    }
}
