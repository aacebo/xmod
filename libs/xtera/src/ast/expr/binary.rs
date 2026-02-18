use crate::Scope;
use crate::ast::{
    BinaryOp, DivisionByZeroError, EvalError, Result, Span, expect_number, is_truthy,
};

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
    pub span: Span,
}

impl BinaryExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        // Short-circuit for logical ops.
        match self.op {
            BinaryOp::And => {
                let left_val = self.left.eval(scope)?;
                if !is_truthy(&left_val) {
                    return Ok(left_val);
                }
                return self.right.eval(scope);
            }
            BinaryOp::Or => {
                let left_val = self.left.eval(scope)?;
                if is_truthy(&left_val) {
                    return Ok(left_val);
                }
                return self.right.eval(scope);
            }
            _ => {}
        }

        let left_val = self.left.eval(scope)?;
        let right_val = self.right.eval(scope)?;

        match self.op {
            BinaryOp::Eq => Ok(xval::valueof!((left_val == right_val))),
            BinaryOp::Ne => Ok(xval::valueof!((left_val != right_val))),
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                Self::eval_comparison(&left_val, self.op, &right_val)
            }
            BinaryOp::Add if left_val.is_string() || right_val.is_string() => {
                Ok(xval::valueof!((format!("{}{}", left_val, right_val))))
            }
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                let l = expect_number(&left_val, self.span)?;
                let r = expect_number(&right_val, self.span)?;
                Self::eval_arithmetic(l, self.op, r, self.span)
            }
            BinaryOp::And | BinaryOp::Or => unreachable!(),
        }
    }

    fn eval_comparison(
        left: &xval::Value,
        op: BinaryOp,
        right: &xval::Value,
    ) -> Result<xval::Value> {
        if left.is_number() && right.is_number() {
            let lf = left.as_number().to_f64();
            let rf = right.as_number().to_f64();
            let result = match op {
                BinaryOp::Lt => lf < rf,
                BinaryOp::Le => lf <= rf,
                BinaryOp::Gt => lf > rf,
                BinaryOp::Ge => lf >= rf,
                _ => unreachable!(),
            };
            Ok(xval::valueof!((result)))
        } else {
            let cmp = left.partial_cmp(right);
            let result = match (op, cmp) {
                (BinaryOp::Lt, Some(std::cmp::Ordering::Less)) => true,
                (BinaryOp::Le, Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)) => true,
                (BinaryOp::Gt, Some(std::cmp::Ordering::Greater)) => true,
                (BinaryOp::Ge, Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)) => {
                    true
                }
                _ => false,
            };
            Ok(xval::valueof!((result)))
        }
    }

    fn eval_arithmetic(
        l: &xval::Number,
        op: BinaryOp,
        r: &xval::Number,
        span: Span,
    ) -> Result<xval::Value> {
        if l.is_float() || r.is_float() {
            let lf = l.to_f64();
            let rf = r.to_f64();
            return match op {
                BinaryOp::Add => Ok(xval::valueof!((lf + rf))),
                BinaryOp::Sub => Ok(xval::valueof!((lf - rf))),
                BinaryOp::Mul => Ok(xval::valueof!((lf * rf))),
                BinaryOp::Div => {
                    if rf == 0.0 {
                        return Err(EvalError::DivisionByZero(DivisionByZeroError { span }));
                    }
                    Ok(xval::valueof!((lf / rf)))
                }
                BinaryOp::Mod => {
                    if rf == 0.0 {
                        return Err(EvalError::DivisionByZero(DivisionByZeroError { span }));
                    }
                    Ok(xval::valueof!((lf % rf)))
                }
                _ => unreachable!(),
            };
        }

        let li = l.to_i64();
        let ri = r.to_i64();

        match op {
            BinaryOp::Add => Ok(xval::valueof!((li.wrapping_add(ri)))),
            BinaryOp::Sub => Ok(xval::valueof!((li.wrapping_sub(ri)))),
            BinaryOp::Mul => Ok(xval::valueof!((li.wrapping_mul(ri)))),
            BinaryOp::Div => {
                if ri == 0 {
                    return Err(EvalError::DivisionByZero(DivisionByZeroError { span }));
                }
                Ok(xval::valueof!((li.wrapping_div(ri))))
            }
            BinaryOp::Mod => {
                if ri == 0 {
                    return Err(EvalError::DivisionByZero(DivisionByZeroError { span }));
                }
                Ok(xval::valueof!((li.wrapping_rem(ri))))
            }
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for BinaryExpr {
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

    fn ident(name: &str) -> Box<Expr> {
        Box::new(Expr::Ident(crate::ast::IdentExpr {
            name: name.to_string(),
            span: Span::new(0, 1),
        }))
    }

    fn binary(left: Box<Expr>, op: BinaryOp, right: Box<Expr>) -> BinaryExpr {
        BinaryExpr {
            left,
            op,
            right,
            span: Span::new(0, 1),
        }
    }

    #[test]
    fn add_ints() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::valueof!(2_i64)),
            BinaryOp::Add,
            val(xval::valueof!(3_i64)),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), 5i64);
    }

    #[test]
    fn float_promotion() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::valueof!(1_i64)),
            BinaryOp::Add,
            val(xval::valueof!(2.5_f64)),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), 3.5f64);
    }

    #[test]
    fn division_by_zero() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::valueof!(10_i64)),
            BinaryOp::Div,
            val(xval::valueof!(0_i64)),
        );
        assert!(matches!(
            expr.eval(&ctx).unwrap_err(),
            EvalError::DivisionByZero(_)
        ));
    }

    #[test]
    fn string_concat() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::valueof!("hello")),
            BinaryOp::Add,
            val(xval::valueof!(" world")),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), "hello world");
    }

    #[test]
    fn comparison() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::valueof!(1_i64)),
            BinaryOp::Lt,
            val(xval::valueof!(2_i64)),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), true);
    }

    #[test]
    fn equality() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::valueof!("a")),
            BinaryOp::Eq,
            val(xval::valueof!("a")),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), true);
    }

    #[test]
    fn logical_and_short_circuit() {
        let ctx = Scope::new();
        let expr = binary(val(xval::valueof!(false)), BinaryOp::And, ident("missing"));
        assert_eq!(expr.eval(&ctx).unwrap(), false);
    }

    #[test]
    fn logical_or_short_circuit() {
        let ctx = Scope::new();
        let expr = binary(val(xval::valueof!(true)), BinaryOp::Or, ident("missing"));
        assert_eq!(expr.eval(&ctx).unwrap(), true);
    }
}
