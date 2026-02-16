use crate::Scope;
use crate::ast::{Result, Span};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpr {
    pub elements: Vec<Expr>,
    pub span: Span,
}

impl ArrayExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let values: Vec<xval::Value> = self
            .elements
            .iter()
            .map(|e| e.eval(scope))
            .collect::<Result<_>>()?;

        Ok(xval::Value::from_array(values))
    }
}
