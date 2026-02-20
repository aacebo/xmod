use crate::Scope;
use crate::ast::{EvalError, Result, Span, UndefinedVariableError};

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr {
    pub name: String,
    pub span: Span,
}

impl IdentExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        scope.var(&self.name).cloned().ok_or_else(|| {
            EvalError::UndefinedVariable(UndefinedVariableError {
                name: self.name.clone(),
            })
            .with_span(self.span.clone())
        })
    }
}

impl std::fmt::Display for IdentExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
