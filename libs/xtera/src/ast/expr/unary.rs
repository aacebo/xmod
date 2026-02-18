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
            UnaryOp::Not => Ok(xval::valueof!((!is_truthy(&val)))),
            UnaryOp::Neg => {
                let n = expect_number(&val, self.span)?;
                if n.is_float() {
                    Ok(xval::valueof!((-n.to_f64())))
                } else {
                    Ok(xval::valueof!((n.to_i64().wrapping_neg())))
                }
            }
        }
    }
}

impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ValueExpr;

    fn val(v: xval::Value) -> Box<Expr> {
        Box::new(Expr::Value(ValueExpr {
            value: v,
            span: Span::new(0, 1),
        }))
    }

    #[test]
    fn not() {
        let ctx = Scope::new();
        let expr = UnaryExpr {
            op: UnaryOp::Not,
            operand: val(xval::valueof!(true)),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), false);
    }

    #[test]
    fn neg() {
        let ctx = Scope::new();
        let expr = UnaryExpr {
            op: UnaryOp::Neg,
            operand: val(xval::valueof!(5_i64)),
            span: Span::new(0, 1),
        };
        assert_eq!(expr.eval(&ctx).unwrap(), -5i64);
    }
}
