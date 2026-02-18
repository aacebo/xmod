use crate::Scope;
use crate::ast::{
    EvalError, IndexOutOfBoundsError, Result, Span, TypeError, value_to_usize, value_type_name,
};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
    pub span: Span,
}

impl IndexExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let obj = self.object.eval(scope)?;
        let idx = self.index.eval(scope)?;

        if !obj.is_array() {
            return Err(EvalError::TypeError(TypeError {
                expected: "array",
                got: value_type_name(&obj),
                span: self.span.clone(),
            }));
        }

        let i = value_to_usize(&idx, self.span.clone())?;
        let arr = obj.as_array();
        arr.index(i).map(|v| v.as_value()).ok_or_else(|| {
            EvalError::IndexOutOfBounds(IndexOutOfBoundsError {
                index: i,
                len: arr.len(),
                span: self.span.clone(),
            })
        })
    }
}

impl std::fmt::Display for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
