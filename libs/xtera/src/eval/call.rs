use crate::Scope;
use crate::ast::{Expr, ExprKind, Span};

use super::error::{EvalError, EvalErrorKind, Result};
use super::eval_expr;

pub fn eval_call(callee: &Expr, args: &[Expr], span: Span, ctx: &Scope) -> Result<xval::Value> {
    let name = match &callee.kind {
        ExprKind::Ident(name) => name.as_str(),
        _ => return Err(EvalError::new(EvalErrorKind::NotCallable, span)),
    };

    let func = ctx
        .func(name)
        .ok_or_else(|| EvalError::new(EvalErrorKind::NotCallable, span))?;

    let evaluated_args: Vec<xval::Value> = args
        .iter()
        .map(|a| eval_expr(a, ctx))
        .collect::<Result<_>>()?;

    func.invoke(&evaluated_args)
}
