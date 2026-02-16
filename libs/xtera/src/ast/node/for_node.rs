use crate::Scope;
use crate::ast::{EvalError, Expr, NotIterableError, Result, Span};

use super::{Node, render_nodes};

#[derive(Debug, Clone, PartialEq)]
pub struct ForNode {
    pub binding: String,
    pub iterable: Expr,
    pub track: Expr,
    pub body: Vec<Node>,
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
            output.push_str(&render_nodes(&self.body, &inner)?);
        }

        Ok(output)
    }
}
