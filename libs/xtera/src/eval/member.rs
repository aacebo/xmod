use crate::Scope;
use crate::ast::{Expr, Span};

use super::error::{EvalError, EvalErrorKind, Result};
use super::{eval_expr, value_type_name};

pub fn eval_member(object: &Expr, field: &str, span: Span, ctx: &Scope) -> Result<xval::Value> {
    let obj = eval_expr(object, ctx)?;
    if !obj.is_struct() {
        return Err(EvalError::new(
            EvalErrorKind::TypeError {
                expected: "struct",
                got: value_type_name(&obj),
            },
            span,
        ));
    }

    obj.as_struct()
        .field(xval::Ident::key(field))
        .map(|v| v.as_value())
        .ok_or_else(|| EvalError::new(EvalErrorKind::UndefinedField(field.to_string()), span))
}
