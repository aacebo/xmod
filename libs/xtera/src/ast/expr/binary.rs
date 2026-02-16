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
            BinaryOp::Eq => Ok(xval::Value::from_bool(left_val == right_val)),
            BinaryOp::Ne => Ok(xval::Value::from_bool(left_val != right_val)),
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                Self::eval_comparison(&left_val, self.op, &right_val)
            }
            BinaryOp::Add if left_val.is_string() || right_val.is_string() => Ok(
                xval::Value::from_string(format!("{}{}", left_val, right_val)),
            ),
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
            Ok(xval::Value::from_bool(result))
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
            Ok(xval::Value::from_bool(result))
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
                BinaryOp::Add => Ok(xval::Value::from_f64(lf + rf)),
                BinaryOp::Sub => Ok(xval::Value::from_f64(lf - rf)),
                BinaryOp::Mul => Ok(xval::Value::from_f64(lf * rf)),
                BinaryOp::Div => {
                    if rf == 0.0 {
                        return Err(EvalError::DivisionByZero(DivisionByZeroError { span }));
                    }
                    Ok(xval::Value::from_f64(lf / rf))
                }
                BinaryOp::Mod => {
                    if rf == 0.0 {
                        return Err(EvalError::DivisionByZero(DivisionByZeroError { span }));
                    }
                    Ok(xval::Value::from_f64(lf % rf))
                }
                _ => unreachable!(),
            };
        }

        let li = l.to_i64();
        let ri = r.to_i64();

        match op {
            BinaryOp::Add => Ok(xval::Value::from_i64(li.wrapping_add(ri))),
            BinaryOp::Sub => Ok(xval::Value::from_i64(li.wrapping_sub(ri))),
            BinaryOp::Mul => Ok(xval::Value::from_i64(li.wrapping_mul(ri))),
            BinaryOp::Div => {
                if ri == 0 {
                    return Err(EvalError::DivisionByZero(DivisionByZeroError { span }));
                }
                Ok(xval::Value::from_i64(li.wrapping_div(ri)))
            }
            BinaryOp::Mod => {
                if ri == 0 {
                    return Err(EvalError::DivisionByZero(DivisionByZeroError { span }));
                }
                Ok(xval::Value::from_i64(li.wrapping_rem(ri)))
            }
            _ => unreachable!(),
        }
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
            val(xval::Value::from_i64(2)),
            BinaryOp::Add,
            val(xval::Value::from_i64(3)),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), 5i64);
    }

    #[test]
    fn float_promotion() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::Value::from_i64(1)),
            BinaryOp::Add,
            val(xval::Value::from_f64(2.5)),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), 3.5f64);
    }

    #[test]
    fn division_by_zero() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::Value::from_i64(10)),
            BinaryOp::Div,
            val(xval::Value::from_i64(0)),
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
            val(xval::Value::from_str("hello")),
            BinaryOp::Add,
            val(xval::Value::from_str(" world")),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), "hello world");
    }

    #[test]
    fn comparison() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::Value::from_i64(1)),
            BinaryOp::Lt,
            val(xval::Value::from_i64(2)),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), true);
    }

    #[test]
    fn equality() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::Value::from_str("a")),
            BinaryOp::Eq,
            val(xval::Value::from_str("a")),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), true);
    }

    #[test]
    fn logical_and_short_circuit() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::Value::from_bool(false)),
            BinaryOp::And,
            ident("missing"),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), false);
    }

    #[test]
    fn logical_or_short_circuit() {
        let ctx = Scope::new();
        let expr = binary(
            val(xval::Value::from_bool(true)),
            BinaryOp::Or,
            ident("missing"),
        );
        assert_eq!(expr.eval(&ctx).unwrap(), true);
    }
}
