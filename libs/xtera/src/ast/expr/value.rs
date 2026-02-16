use crate::Scope;
use crate::ast::{Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct ValueExpr {
    pub value: xval::Value,
    pub span: Span,
}

impl ValueExpr {
    pub fn eval(&self, _scope: &Scope) -> Result<xval::Value> {
        Ok(self.value.clone())
    }
}
