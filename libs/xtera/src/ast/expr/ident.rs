use crate::Scope;
use crate::ast::{EvalError, Result, Span, UndefinedVariableError};

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr {
    pub name: String,
    pub span: Span,
}

impl IdentExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        scope.var(&self.name).cloned().ok_or_else(|| {
            EvalError::UndefinedVariable(UndefinedVariableError {
                name: self.name.clone(),
                span: self.span,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_found() {
        let mut ctx = Scope::new();
        ctx.set_var("x", xval::Value::from_i64(10));
        let expr = IdentExpr {
            name: "x".to_string(),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), 10i64);
    }

    #[test]
    fn eval_undefined() {
        let ctx = Scope::new();
        let expr = IdentExpr {
            name: "x".to_string(),
            span: Span::new(0, 1),
        };
        let err = expr.eval(&ctx).unwrap_err();
        assert!(matches!(err, EvalError::UndefinedVariable(_)));
    }
}
