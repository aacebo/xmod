pub mod binary;
pub mod call;
mod error;
pub mod index;
pub mod literal;
pub mod member;
pub mod pipe;
pub mod unary;

pub use error::*;

use std::collections::HashMap;

use crate::Scope;
use crate::ast::{Expr, ExprKind, Span};

pub fn eval_expr(expr: &Expr, ctx: &Scope) -> Result<xval::Value> {
    match &expr.kind {
        ExprKind::Literal(lit) => literal::eval_literal(lit),
        ExprKind::Ident(name) => ctx.var(name).cloned().ok_or_else(|| {
            EvalError::new(EvalErrorKind::UndefinedVariable(name.clone()), expr.span)
        }),
        ExprKind::Member { object, field } => member::eval_member(object, field, expr.span, ctx),
        ExprKind::Index { object, index } => index::eval_index(object, index, expr.span, ctx),
        ExprKind::Call { callee, args } => call::eval_call(callee, args, expr.span, ctx),
        ExprKind::Pipe { value, name, args } => pipe::eval_pipe(value, name, args, expr.span, ctx),
        ExprKind::Binary { left, op, right } => {
            binary::eval_binary(left, *op, right, expr.span, ctx)
        }
        ExprKind::Unary { op, operand } => unary::eval_unary(*op, operand, expr.span, ctx),
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

// ── shared helpers ──

pub fn is_truthy(val: &xval::Value) -> bool {
    match val {
        xval::Value::Null => false,
        xval::Value::Bool(b) => b.to_bool(),
        xval::Value::Number(n) => n.to_f64() != 0.0,
        xval::Value::String(s) => !s.as_str().is_empty(),
        xval::Value::Object(o) => !o.is_empty(),
    }
}

pub(crate) fn expect_number<'a>(val: &'a xval::Value, span: Span) -> Result<&'a xval::Number> {
    match val {
        xval::Value::Number(n) => Ok(n),
        other => Err(EvalError::new(
            EvalErrorKind::TypeError {
                expected: "number",
                got: value_type_name(other),
            },
            span,
        )),
    }
}

pub(crate) fn value_to_usize(val: &xval::Value, span: Span) -> Result<usize> {
    let n = expect_number(val, span)?;
    let v = n.to_i64();
    if v >= 0 {
        Ok(v as usize)
    } else {
        Err(EvalError::new(EvalErrorKind::InvalidIndex, span))
    }
}

pub(crate) fn value_type_name(val: &xval::Value) -> String {
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
    use crate::ast::{BinaryOp, Expr, ExprKind, Literal, Span, UnaryOp};

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
