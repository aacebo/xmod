use std::collections::HashMap;

use crate::Scope;
use crate::ast::{Result, Span};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExpr {
    pub entries: Vec<(String, Expr)>,
    pub span: Span,
}

impl ObjectExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let mut map = HashMap::new();
        for (key, val_expr) in &self.entries {
            map.insert(xval::Ident::key(key), val_expr.eval(scope)?);
        }
        Ok(xval::Value::from_struct(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ValueExpr;

    #[test]
    fn eval_object_literal() {
        let ctx = Scope::new();
        let expr = ObjectExpr {
            entries: vec![
                (
                    "a".into(),
                    Expr::Value(ValueExpr {
                        value: xval::Value::from_i64(1),
                        span: Span::new(0, 1),
                    }),
                ),
                (
                    "b".into(),
                    Expr::Value(ValueExpr {
                        value: xval::Value::from_str("two"),
                        span: Span::new(0, 1),
                    }),
                ),
            ],
            span: Span::new(0, 1),
        };
        let result = expr.eval(&ctx).unwrap();
        assert!(result.is_struct());
        assert_eq!(result.as_struct().len(), 2);
    }
}
