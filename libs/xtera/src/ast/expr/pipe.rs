use crate::Scope;
use crate::ast::{EvalError, Result, Span, UndefinedPipeError};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct PipeExpr {
    pub value: Box<Expr>,
    pub name: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl PipeExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let val = self.value.eval(scope)?;
        let evaluated_args: Vec<xval::Value> = self
            .args
            .iter()
            .map(|a| a.eval(scope))
            .collect::<Result<_>>()?;

        let pipe = scope.pipe(&self.name).ok_or_else(|| {
            EvalError::UndefinedPipe(UndefinedPipeError {
                name: self.name.clone(),
                span: self.span.clone(),
            })
        })?;

        pipe.invoke(&val, &evaluated_args)
    }
}

impl std::fmt::Display for PipeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
