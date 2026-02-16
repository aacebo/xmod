use crate::Scope;
use crate::ast::{Expr, Span};

use super::error::{EvalError, EvalErrorKind, Result};
use super::eval_expr;

pub fn eval_pipe(
    value: &Expr,
    name: &str,
    args: &[Expr],
    span: Span,
    ctx: &Scope,
) -> Result<xval::Value> {
    let val = eval_expr(value, ctx)?;
    let evaluated_args: Vec<xval::Value> = args
        .iter()
        .map(|a| eval_expr(a, ctx))
        .collect::<Result<_>>()?;

    let pipe = ctx
        .pipe(name)
        .ok_or_else(|| EvalError::new(EvalErrorKind::UndefinedPipe(name.to_string()), span))?;

    pipe.invoke(&val, &evaluated_args)
}
