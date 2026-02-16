use crate::Scope;
use crate::ast::{Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct ValueExpr {
    pub value: xval::Value,
    pub span: Span,
}

impl ValueExpr {
    pub fn eval(&self, _scope: &Scope) -> Result<xval::Value> {
        Ok(self.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_null() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::Value::Null,
            span: Span::new(0, 1),
        };
        assert!(expr.eval(&ctx).unwrap().is_null());
    }

    #[test]
    fn eval_bool() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::Value::from_bool(true),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), true);
    }

    #[test]
    fn eval_int() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::Value::from_i64(42),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), 42i64);
    }

    #[test]
    fn eval_float() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::Value::from_f64(3.14),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), 3.14f64);
    }

    #[test]
    fn eval_string() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::Value::from_str("hello"),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), "hello");
    }
}
