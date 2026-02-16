use crate::ast::Literal;

use super::Result;

pub fn eval_literal(lit: &Literal) -> Result<xval::Value> {
    Ok(match lit {
        Literal::Null => xval::Value::Null,
        Literal::Bool(b) => xval::Value::from_bool(*b),
        Literal::Int(n) => xval::Value::from_i64(*n),
        Literal::Float(n) => xval::Value::from_f64(*n),
        Literal::String(s) => xval::Value::from_string(s.clone()),
    })
}
