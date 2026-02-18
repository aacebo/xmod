use crate::Scope;
use crate::ast::{EvalError, Result, Span, TypeError, UndefinedFieldError, value_type_name};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpr {
    pub object: Box<Expr>,
    pub field: String,
    pub span: Span,
}

impl MemberExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let obj = self.object.eval(scope)?;
        if !obj.is_struct() {
            return Err(EvalError::TypeError(TypeError {
                expected: "struct",
                got: value_type_name(&obj),
                span: self.span,
            }));
        }

        obj.as_struct()
            .field(xval::Ident::key(&self.field))
            .map(|v| v.as_value())
            .ok_or_else(|| {
                EvalError::UndefinedField(UndefinedFieldError {
                    name: self.field.clone(),
                    span: self.span,
                })
            })
    }
}

impl std::fmt::Display for MemberExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
