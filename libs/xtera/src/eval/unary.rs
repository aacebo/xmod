use crate::Scope;
use crate::ast::{Expr, Span, UnaryOp};

use super::error::Result;
use super::{eval_expr, expect_number, is_truthy};

pub fn eval_unary(op: UnaryOp, operand: &Expr, span: Span, ctx: &Scope) -> Result<xval::Value> {
    let val = eval_expr(operand, ctx)?;
    match op {
        UnaryOp::Not => Ok(xval::Value::from_bool(!is_truthy(&val))),
        UnaryOp::Neg => {
            let n = expect_number(&val, span)?;
            if n.is_float() {
                Ok(xval::Value::from_f64(-n.to_f64()))
            } else {
                Ok(xval::Value::from_i64(n.to_i64().wrapping_neg()))
            }
        }
    }
}
