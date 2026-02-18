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

impl std::fmt::Display for ForNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
