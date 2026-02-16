use crate::Scope;
use crate::ast::{EvalError, Expr, NotIterableError, Result, Span};

use super::BlockNode;

#[derive(Debug, Clone, PartialEq)]
pub struct ForNode {
    pub binding: String,
    pub iterable: Expr,
    pub track: Expr,
    pub body: BlockNode,
    pub span: Span,
}

impl ForNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let iterable = self.iterable.eval(scope)?;

        if !iterable.is_array() {
            return Err(EvalError::NotIterable(NotIterableError {
                span: self.iterable.span(),
            }));
        }

        let arr = iterable.as_array();
        let mut output = String::new();

        for item in arr.items() {
            let mut inner = scope.clone();
            inner.set_var(&self.binding, item.as_value());
            output.push_str(&self.body.render(&inner)?);
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{IdentExpr, InterpNode, Node, TextNode};

    fn ident_expr(name: &str) -> Expr {
        Expr::Ident(IdentExpr {
            name: name.into(),
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
    fn render_array() {
        let mut scope = Scope::new();
        scope.set_var(
            "items",
            xval::Value::from_array(vec![
                xval::Value::from_i64(1),
                xval::Value::from_i64(2),
                xval::Value::from_i64(3),
            ]),
        );

        let node = ForNode {
            binding: "n".into(),
            iterable: ident_expr("items"),
            track: ident_expr("n"),
            body: block(vec![
                text("["),
                Node::Interp(InterpNode {
                    expr: ident_expr("n"),
                    span: Span::new(0, 1),
                }),
                text("]"),
            ]),
            span: Span::new(0, 1),
        };
        assert_eq!(node.render(&scope).unwrap(), "[1][2][3]");
    }

    #[test]
    fn render_empty_array() {
        let mut scope = Scope::new();
        scope.set_var("items", xval::Value::from_array(vec![]));

        let node = ForNode {
            binding: "n".into(),
            iterable: ident_expr("items"),
            track: ident_expr("n"),
            body: block(vec![text("x")]),
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "");
    }

    #[test]
    fn render_not_iterable() {
        let mut scope = Scope::new();
        scope.set_var("items", xval::Value::from_i64(42));

        let node = ForNode {
            binding: "n".into(),
            iterable: ident_expr("items"),
            track: ident_expr("n"),
            body: block(vec![text("x")]),
            span: Span::new(0, 1),
        };

        let err = node.render(&scope).unwrap_err();
        assert!(matches!(err, EvalError::NotIterable(_)));
    }
}
