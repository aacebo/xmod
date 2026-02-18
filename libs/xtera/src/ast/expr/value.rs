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

impl std::fmt::Display for ValueExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_null() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::valueof!(null),
            span: Span::new(0, 1),
        };
        assert!(expr.eval(&ctx).unwrap().is_null());
    }

    #[test]
    fn eval_bool() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::valueof!(true),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), true);
    }

    #[test]
    fn eval_int() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::valueof!(42_i64),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), 42i64);
    }

    #[test]
    fn eval_float() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::valueof!(3.14_f64),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), 3.14f64);
    }

    #[test]
    fn eval_string() {
        let ctx = Scope::new();
        let expr = ValueExpr {
            value: xval::valueof!("hello"),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), "hello");
    }
}
