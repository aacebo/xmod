use crate::Scope;
use crate::ast::{Result, Span};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpr {
    pub elements: Vec<Expr>,
    pub span: Span,
}

impl ArrayExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let values: Vec<xval::Value> = self
            .elements
            .iter()
            .map(|e| e.eval(scope))
            .collect::<Result<_>>()?;

        Ok(xval::Value::from_array(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ValueExpr;

    #[test]
    fn eval_array_literal() {
        let ctx = Scope::new();
        let expr = ArrayExpr {
            elements: vec![
                Expr::Value(ValueExpr {
                    value: xval::Value::from_i64(1),
                    span: Span::new(0, 1),
                }),
                Expr::Value(ValueExpr {
                    value: xval::Value::from_i64(2),
                    span: Span::new(0, 1),
                }),
            ],
            span: Span::new(0, 1),
        };

        let result = expr.eval(&ctx).unwrap();
        assert!(result.is_array());
        assert_eq!(result.as_array().len(), 2);
    }
}
