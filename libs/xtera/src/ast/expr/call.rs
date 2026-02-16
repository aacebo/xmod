use crate::Scope;
use crate::ast::{EvalError, NotCallableError, Result, Span};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl CallExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let name = match &*self.callee {
            Expr::Ident(ident) => ident.name.as_str(),
            _ => return Err(EvalError::NotCallable(NotCallableError { span: self.span })),
        };

        let func = scope
            .func(name)
            .ok_or_else(|| EvalError::NotCallable(NotCallableError { span: self.span }))?;

        let evaluated_args: Vec<xval::Value> = self
            .args
            .iter()
            .map(|a| a.eval(scope))
            .collect::<Result<_>>()?;

        func.invoke(&evaluated_args)
    }
}
