use crate::Scope;
use crate::ast::{Expr, Span};

use super::error::{EvalError, EvalErrorKind, Result};
use super::{eval_expr, value_to_usize, value_type_name};

pub fn eval_index(object: &Expr, index: &Expr, span: Span, ctx: &Scope) -> Result<xval::Value> {
    let obj = eval_expr(object, ctx)?;
    let idx = eval_expr(index, ctx)?;

    if !obj.is_array() {
        return Err(EvalError::new(
            EvalErrorKind::TypeError {
                expected: "array",
                got: value_type_name(&obj),
            },
            span,
        ));
    }

    let i = value_to_usize(&idx, span)?;
    let arr = obj.as_array();
    arr.index(i).map(|v| v.as_value()).ok_or_else(|| {
        EvalError::new(
            EvalErrorKind::IndexOutOfBounds {
                index: i,
                len: arr.len(),
            },
            span,
        )
    })
}
