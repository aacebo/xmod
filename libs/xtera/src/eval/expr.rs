use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, ExprKind, Literal, Span, UnaryOp};

use super::error::{EvalError, EvalErrorKind, Result};
use crate::Scope;

pub fn eval_expr(expr: &Expr, ctx: &Scope) -> Result<xval::Value> {
    match &expr.kind {
        ExprKind::Literal(lit) => eval_literal(lit),
        ExprKind::Ident(name) => ctx.var(name).cloned().ok_or_else(|| {
            EvalError::new(EvalErrorKind::UndefinedVariable(name.clone()), expr.span)
        }),

        ExprKind::Member { object, field } => eval_member(object, field, expr.span, ctx),
        ExprKind::Index { object, index } => eval_index(object, index, expr.span, ctx),
        ExprKind::Call { callee, args } => eval_call(callee, args, expr.span, ctx),
        ExprKind::Pipe { value, name, args } => eval_pipe(value, name, args, expr.span, ctx),
        ExprKind::Binary { left, op, right } => eval_binary(left, *op, right, expr.span, ctx),
        ExprKind::Unary { op, operand } => eval_unary(*op, operand, expr.span, ctx),
        ExprKind::Array(elements) => {
            let values: Vec<xval::Value> = elements
                .iter()
                .map(|e| eval_expr(e, ctx))
                .collect::<Result<_>>()?;

            Ok(xval::Value::from_array(values))
        }

        ExprKind::Object(entries) => {
            let mut map = HashMap::new();
            for (key, val_expr) in entries {
                map.insert(xval::Ident::key(key), eval_expr(val_expr, ctx)?);
            }

            Ok(xval::Value::from_struct(map))
        }
    }
}

fn eval_literal(lit: &Literal) -> Result<xval::Value> {
    Ok(match lit {
        Literal::Null => xval::Value::Null,
        Literal::Bool(b) => xval::Value::from_bool(*b),
        Literal::Int(n) => xval::Value::from_i64(*n),
        Literal::Float(n) => xval::Value::from_f64(*n),
        Literal::String(s) => xval::Value::from_string(s.clone()),
    })
}

fn eval_member(object: &Expr, field: &str, span: Span, ctx: &Scope) -> Result<xval::Value> {
    let obj = eval_expr(object, ctx)?;
    if !obj.is_struct() {
        return Err(EvalError::new(
            EvalErrorKind::TypeError {
                expected: "struct",
                got: value_type_name(&obj),
            },
            span,
        ));
    }

    obj.as_struct()
        .field(xval::Ident::key(field))
        .map(|v| v.as_value())
        .ok_or_else(|| EvalError::new(EvalErrorKind::UndefinedField(field.to_string()), span))
}

fn eval_index(object: &Expr, index: &Expr, span: Span, ctx: &Scope) -> Result<xval::Value> {
    let obj = eval_expr(object, ctx)?;
    let idx = eval_expr(index, ctx)?;

    if !obj.is_array() {
        return Err(EvalError::new(
            EvalErrorKind::TypeError {
                expected: "array",
                got: value_type_name(&obj),
            },
            span,
        ));
    }

    let i = value_to_usize(&idx, span)?;
    let arr = obj.as_array();
    arr.index(i).map(|v| v.as_value()).ok_or_else(|| {
        EvalError::new(
            EvalErrorKind::IndexOutOfBounds {
                index: i,
                len: arr.len(),
            },
            span,
        )
    })
}

fn eval_pipe(
    value: &Expr,
    name: &str,
    args: &[Expr],
    span: Span,
    ctx: &Scope,
) -> Result<xval::Value> {
    let val = eval_expr(value, ctx)?;
    let evaluated_args: Vec<xval::Value> = args
        .iter()
        .map(|a| eval_expr(a, ctx))
        .collect::<Result<_>>()?;

    let pipe = ctx
        .pipe(name)
        .ok_or_else(|| EvalError::new(EvalErrorKind::UndefinedPipe(name.to_string()), span))?;

    pipe.invoke(&val, &evaluated_args)
}

fn eval_call(callee: &Expr, args: &[Expr], span: Span, ctx: &Scope) -> Result<xval::Value> {
    let name = match &callee.kind {
        ExprKind::Ident(name) => name.as_str(),
        _ => return Err(EvalError::new(EvalErrorKind::NotCallable, span)),
    };

    let func = ctx
        .func(name)
        .ok_or_else(|| EvalError::new(EvalErrorKind::NotCallable, span))?;

    let evaluated_args: Vec<xval::Value> = args
        .iter()
        .map(|a| eval_expr(a, ctx))
        .collect::<Result<_>>()?;

    func.invoke(&evaluated_args)
}

fn eval_binary(
    left: &Expr,
    op: BinaryOp,
    right: &Expr,
    span: Span,
    ctx: &Scope,
) -> Result<xval::Value> {
    // Short-circuit for logical ops.
    match op {
        BinaryOp::And => {
            let left_val = eval_expr(left, ctx)?;
            if !is_truthy(&left_val) {
                return Ok(left_val);
            }
            return eval_expr(right, ctx);
        }

        BinaryOp::Or => {
            let left_val = eval_expr(left, ctx)?;
            if is_truthy(&left_val) {
                return Ok(left_val);
            }
            return eval_expr(right, ctx);
        }

        _ => {}
    }

    let left_val = eval_expr(left, ctx)?;
    let right_val = eval_expr(right, ctx)?;

    match op {
        // Equality uses Value's PartialEq.
        BinaryOp::Eq => Ok(xval::Value::from_bool(left_val == right_val)),
        BinaryOp::Ne => Ok(xval::Value::from_bool(left_val != right_val)),

        // Ordering comparisons.
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
            eval_comparison(&left_val, op, &right_val, span)
        }

        // Arithmetic — string concat for Add if either side is a string.
        BinaryOp::Add if left_val.is_string() || right_val.is_string() => Ok(
            xval::Value::from_string(format!("{}{}", left_val, right_val)),
        ),

        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
            let l = value_to_numeric(&left_val, span)?;
            let r = value_to_numeric(&right_val, span)?;
            eval_arithmetic(l, op, r, span)
        }

        // Already handled above.
        BinaryOp::And | BinaryOp::Or => unreachable!(),
    }
}

fn eval_comparison(
    left: &xval::Value,
    op: BinaryOp,
    right: &xval::Value,
    span: Span,
) -> Result<xval::Value> {
    if left.is_number() && right.is_number() {
        let l = value_to_numeric(left, span)?;
        let r = value_to_numeric(right, span)?;
        let (lf, rf) = promote_to_f64(l, r);
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
            (BinaryOp::Ge, Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)) => true,
            _ => false,
        };
        Ok(xval::Value::from_bool(result))
    }
}

fn eval_unary(op: UnaryOp, operand: &Expr, span: Span, ctx: &Scope) -> Result<xval::Value> {
    let val = eval_expr(operand, ctx)?;
    match op {
        UnaryOp::Not => Ok(xval::Value::from_bool(!is_truthy(&val))),
        UnaryOp::Neg => {
            let n = value_to_numeric(&val, span)?;
            match n {
                Numeric::Int(v) => Ok(xval::Value::from_i64(-v)),
                Numeric::Float(v) => Ok(xval::Value::from_f64(-v)),
            }
        }
    }
}

// ── helpers ──

pub fn is_truthy(val: &xval::Value) -> bool {
    match val {
        xval::Value::Null => false,
        xval::Value::Bool(b) => b.to_bool(),
        xval::Value::Number(n) => {
            use xval::{Float, Int, UInt};
            match n {
                xval::Number::Int(i) => match i {
                    Int::I8(v) => *v != 0,
                    Int::I16(v) => *v != 0,
                    Int::I32(v) => *v != 0,
                    Int::I64(v) => *v != 0,
                },
                xval::Number::UInt(u) => match u {
                    UInt::U8(v) => *v != 0,
                    UInt::U16(v) => *v != 0,
                    UInt::U32(v) => *v != 0,
                    UInt::U64(v) => *v != 0,
                },
                xval::Number::Float(f) => match f {
                    Float::F32(v) => *v != 0.0,
                    Float::F64(v) => *v != 0.0,
                },
            }
        }
        xval::Value::String(s) => !s.as_str().is_empty(),
        xval::Value::Object(o) => !o.is_empty(),
    }
}

enum Numeric {
    Int(i64),
    Float(f64),
}

fn value_to_numeric(val: &xval::Value, span: Span) -> Result<Numeric> {
    match val {
        xval::Value::Number(n) => {
            use xval::{Float, Int, UInt};
            match n {
                xval::Number::Int(i) => {
                    let v = match i {
                        Int::I8(v) => *v as i64,
                        Int::I16(v) => *v as i64,
                        Int::I32(v) => *v as i64,
                        Int::I64(v) => *v,
                    };
                    Ok(Numeric::Int(v))
                }
                xval::Number::UInt(u) => {
                    let v = match u {
                        UInt::U8(v) => *v as i64,
                        UInt::U16(v) => *v as i64,
                        UInt::U32(v) => *v as i64,
                        UInt::U64(v) => i64::try_from(*v)
                            .map_err(|_| EvalError::new(EvalErrorKind::Overflow, span))?,
                    };
                    Ok(Numeric::Int(v))
                }
                xval::Number::Float(f) => {
                    let v = match f {
                        Float::F32(v) => *v as f64,
                        Float::F64(v) => *v,
                    };
                    Ok(Numeric::Float(v))
                }
            }
        }
        other => Err(EvalError::new(
            EvalErrorKind::TypeError {
                expected: "number",
                got: value_type_name(other),
            },
            span,
        )),
    }
}

fn value_to_usize(val: &xval::Value, span: Span) -> Result<usize> {
    match value_to_numeric(val, span)? {
        Numeric::Int(v) if v >= 0 => Ok(v as usize),
        Numeric::Int(_) | Numeric::Float(_) => {
            Err(EvalError::new(EvalErrorKind::InvalidIndex, span))
        }
    }
}

fn promote_to_f64(a: Numeric, b: Numeric) -> (f64, f64) {
    let af = match a {
        Numeric::Int(v) => v as f64,
        Numeric::Float(v) => v,
    };
    let bf = match b {
        Numeric::Int(v) => v as f64,
        Numeric::Float(v) => v,
    };
    (af, bf)
}

fn eval_arithmetic(l: Numeric, op: BinaryOp, r: Numeric, span: Span) -> Result<xval::Value> {
    // If either is float, promote both to f64.
    if matches!(l, Numeric::Float(_)) || matches!(r, Numeric::Float(_)) {
        let (lf, rf) = promote_to_f64(l, r);
        return match op {
            BinaryOp::Add => Ok(xval::Value::from_f64(lf + rf)),
            BinaryOp::Sub => Ok(xval::Value::from_f64(lf - rf)),
            BinaryOp::Mul => Ok(xval::Value::from_f64(lf * rf)),
            BinaryOp::Div => {
                if rf == 0.0 {
                    return Err(EvalError::new(EvalErrorKind::DivisionByZero, span));
                }
                Ok(xval::Value::from_f64(lf / rf))
            }
            BinaryOp::Mod => {
                if rf == 0.0 {
                    return Err(EvalError::new(EvalErrorKind::DivisionByZero, span));
                }
                Ok(xval::Value::from_f64(lf % rf))
            }
            _ => unreachable!(),
        };
    }

    // Both are integers.
    let (Numeric::Int(li), Numeric::Int(ri)) = (l, r) else {
        unreachable!()
    };

    let overflow = || EvalError::new(EvalErrorKind::Overflow, span);

    match op {
        BinaryOp::Add => Ok(xval::Value::from_i64(
            li.checked_add(ri).ok_or_else(overflow)?,
        )),
        BinaryOp::Sub => Ok(xval::Value::from_i64(
            li.checked_sub(ri).ok_or_else(overflow)?,
        )),
        BinaryOp::Mul => Ok(xval::Value::from_i64(
            li.checked_mul(ri).ok_or_else(overflow)?,
        )),
        BinaryOp::Div => {
            if ri == 0 {
                return Err(EvalError::new(EvalErrorKind::DivisionByZero, span));
            }
            Ok(xval::Value::from_i64(
                li.checked_div(ri).ok_or_else(overflow)?,
            ))
        }
        BinaryOp::Mod => {
            if ri == 0 {
                return Err(EvalError::new(EvalErrorKind::DivisionByZero, span));
            }
            Ok(xval::Value::from_i64(
                li.checked_rem(ri).ok_or_else(overflow)?,
            ))
        }
        _ => unreachable!(),
    }
}

fn value_type_name(val: &xval::Value) -> String {
    match val {
        xval::Value::Null => "null".to_string(),
        xval::Value::Bool(_) => "bool".to_string(),
        xval::Value::Number(_) => "number".to_string(),
        xval::Value::String(_) => "string".to_string(),
        xval::Value::Object(o) => o.name().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expr, ExprKind, Literal, Span};

    fn span() -> Span {
        Span::new(0, 1)
    }

    fn lit_expr(lit: Literal) -> Expr {
        Expr::new(ExprKind::Literal(lit), span())
    }

    fn ident_expr(name: &str) -> Expr {
        Expr::new(ExprKind::Ident(name.to_string()), span())
    }

    fn binary_expr(left: Expr, op: BinaryOp, right: Expr) -> Expr {
        Expr::new(
            ExprKind::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            },
            span(),
        )
    }

    #[test]
    fn eval_literal_null() {
        let ctx = Scope::new();
        let result = eval_expr(&lit_expr(Literal::Null), &ctx).unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn eval_literal_bool() {
        let ctx = Scope::new();
        let result = eval_expr(&lit_expr(Literal::Bool(true)), &ctx).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn eval_literal_int() {
        let ctx = Scope::new();
        let result = eval_expr(&lit_expr(Literal::Int(42)), &ctx).unwrap();
        assert_eq!(result, 42i64);
    }

    #[test]
    fn eval_literal_float() {
        let ctx = Scope::new();
        let result = eval_expr(&lit_expr(Literal::Float(3.14)), &ctx).unwrap();
        assert_eq!(result, 3.14f64);
    }

    #[test]
    fn eval_literal_string() {
        let ctx = Scope::new();
        let result = eval_expr(&lit_expr(Literal::String("hello".into())), &ctx).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn eval_ident_found() {
        let mut ctx = Scope::new();
        ctx.set_var("x", xval::Value::from_i64(10));
        let result = eval_expr(&ident_expr("x"), &ctx).unwrap();
        assert_eq!(result, 10i64);
    }

    #[test]
    fn eval_ident_undefined() {
        let ctx = Scope::new();
        let err = eval_expr(&ident_expr("x"), &ctx).unwrap_err();
        assert_eq!(err.kind, EvalErrorKind::UndefinedVariable("x".into()));
    }

    #[test]
    fn eval_add_ints() {
        let ctx = Scope::new();
        let expr = binary_expr(
            lit_expr(Literal::Int(2)),
            BinaryOp::Add,
            lit_expr(Literal::Int(3)),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, 5i64);
    }

    #[test]
    fn eval_float_promotion() {
        let ctx = Scope::new();
        let expr = binary_expr(
            lit_expr(Literal::Int(1)),
            BinaryOp::Add,
            lit_expr(Literal::Float(2.5)),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, 3.5f64);
    }

    #[test]
    fn eval_division_by_zero() {
        let ctx = Scope::new();
        let expr = binary_expr(
            lit_expr(Literal::Int(10)),
            BinaryOp::Div,
            lit_expr(Literal::Int(0)),
        );
        let err = eval_expr(&expr, &ctx).unwrap_err();
        assert_eq!(err.kind, EvalErrorKind::DivisionByZero);
    }

    #[test]
    fn eval_string_concat() {
        let ctx = Scope::new();
        let expr = binary_expr(
            lit_expr(Literal::String("hello".into())),
            BinaryOp::Add,
            lit_expr(Literal::String(" world".into())),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn eval_comparison() {
        let ctx = Scope::new();
        let expr = binary_expr(
            lit_expr(Literal::Int(1)),
            BinaryOp::Lt,
            lit_expr(Literal::Int(2)),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn eval_equality() {
        let ctx = Scope::new();
        let expr = binary_expr(
            lit_expr(Literal::String("a".into())),
            BinaryOp::Eq,
            lit_expr(Literal::String("a".into())),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn eval_logical_and_short_circuit() {
        let ctx = Scope::new();
        // false && (undefined var) should not error
        let expr = binary_expr(
            lit_expr(Literal::Bool(false)),
            BinaryOp::And,
            ident_expr("missing"),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn eval_logical_or_short_circuit() {
        let ctx = Scope::new();
        // true || (undefined var) should not error
        let expr = binary_expr(
            lit_expr(Literal::Bool(true)),
            BinaryOp::Or,
            ident_expr("missing"),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn eval_unary_not() {
        let ctx = Scope::new();
        let expr = Expr::new(
            ExprKind::Unary {
                op: UnaryOp::Not,
                operand: Box::new(lit_expr(Literal::Bool(true))),
            },
            span(),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn eval_unary_neg() {
        let ctx = Scope::new();
        let expr = Expr::new(
            ExprKind::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(lit_expr(Literal::Int(5))),
            },
            span(),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, -5i64);
    }

    struct DoublePipe;
    impl crate::Pipe for DoublePipe {
        fn invoke(&self, val: &xval::Value, _args: &[xval::Value]) -> Result<xval::Value> {
            let n = match val {
                xval::Value::Number(_) => val.to_i64(),
                _ => {
                    return Err(EvalError::new(
                        EvalErrorKind::TypeError {
                            expected: "number",
                            got: "other".into(),
                        },
                        Span::new(0, 0),
                    ));
                }
            };
            Ok(xval::Value::from_i64(n * 2))
        }
    }

    #[test]
    fn eval_pipe() {
        let mut ctx = Scope::new();
        ctx.set_pipe("double", DoublePipe);
        let expr = Expr::new(
            ExprKind::Pipe {
                value: Box::new(lit_expr(Literal::Int(5))),
                name: "double".into(),
                args: vec![],
            },
            span(),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert_eq!(result, 10i64);
    }

    #[test]
    fn eval_undefined_pipe() {
        let ctx = Scope::new();
        let expr = Expr::new(
            ExprKind::Pipe {
                value: Box::new(lit_expr(Literal::Int(5))),
                name: "nope".into(),
                args: vec![],
            },
            span(),
        );
        let err = eval_expr(&expr, &ctx).unwrap_err();
        assert_eq!(err.kind, EvalErrorKind::UndefinedPipe("nope".into()));
    }

    #[test]
    fn eval_array_literal() {
        let ctx = Scope::new();
        let expr = Expr::new(
            ExprKind::Array(vec![lit_expr(Literal::Int(1)), lit_expr(Literal::Int(2))]),
            span(),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert!(result.is_array());
        assert_eq!(result.as_array().len(), 2);
    }

    #[test]
    fn eval_object_literal() {
        let ctx = Scope::new();
        let expr = Expr::new(
            ExprKind::Object(vec![
                ("a".into(), lit_expr(Literal::Int(1))),
                ("b".into(), lit_expr(Literal::String("two".into()))),
            ]),
            span(),
        );
        let result = eval_expr(&expr, &ctx).unwrap();
        assert!(result.is_struct());
        assert_eq!(result.as_struct().len(), 2);
    }

    #[test]
    fn truthiness() {
        assert!(!is_truthy(&xval::Value::Null));
        assert!(!is_truthy(&xval::Value::from_bool(false)));
        assert!(is_truthy(&xval::Value::from_bool(true)));
        assert!(!is_truthy(&xval::Value::from_i64(0)));
        assert!(is_truthy(&xval::Value::from_i64(1)));
        assert!(!is_truthy(&xval::Value::from_f64(0.0)));
        assert!(is_truthy(&xval::Value::from_f64(0.1)));
        assert!(!is_truthy(&xval::Value::from_str("")));
        assert!(is_truthy(&xval::Value::from_str("x")));
    }
}
