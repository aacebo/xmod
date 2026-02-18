use crate::Scope;
use crate::ast::{Expr, Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct InterpNode {
    pub expr: Expr,
    pub span: Span,
}

impl InterpNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let val = self.expr.eval(scope)?;
        Ok(val.to_string())
    }
}

impl std::fmt::Display for InterpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
