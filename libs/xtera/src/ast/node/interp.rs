use crate::Scope;
use crate::ast::{Expr, Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct InterpNode {
    pub expr: Expr,
    pub span: Span,
}

impl InterpNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let val = self.expr.eval(scope)?;
        Ok(val.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ValueExpr;

    fn interp(v: xval::Value) -> InterpNode {
        InterpNode {
            expr: Expr::Value(ValueExpr {
                value: v,
                span: Span::new(0, 1),
            }),
            span: Span::new(0, 1),
        }
    }

    #[test]
    fn render_int() {
        let scope = Scope::new();
        assert_eq!(
            interp(xval::Value::from_i64(42)).render(&scope).unwrap(),
            "42"
        );
    }

    #[test]
    fn render_string() {
        let scope = Scope::new();
        assert_eq!(
            interp(xval::Value::from_str("hi")).render(&scope).unwrap(),
            "hi"
        );
    }

    #[test]
    fn render_bool() {
        let scope = Scope::new();
        assert_eq!(
            interp(xval::Value::from_bool(true)).render(&scope).unwrap(),
            "true"
        );
    }
}
