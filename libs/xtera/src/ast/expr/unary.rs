use crate::Scope;
use crate::ast::{Result, Span, UnaryOp, expect_number, is_truthy};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub operand: Box<Expr>,
    pub span: Span,
}

impl UnaryExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let val = self.operand.eval(scope)?;
        match self.op {
            UnaryOp::Not => Ok(xval::Value::from_bool(!is_truthy(&val))),
            UnaryOp::Neg => {
                let n = expect_number(&val, self.span)?;
                if n.is_float() {
                    Ok(xval::Value::from_f64(-n.to_f64()))
                } else {
                    Ok(xval::Value::from_i64(n.to_i64().wrapping_neg()))
                }
            }
        }
    }
}
