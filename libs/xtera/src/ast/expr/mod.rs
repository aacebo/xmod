mod array;
mod binary;
mod call;
mod ident;
mod index;
mod member;
mod object;
mod pipe;
mod unary;
mod value;

pub use array::*;
pub use binary::*;
pub use call::*;
pub use ident::*;
pub use index::*;
pub use member::*;
pub use object::*;
pub use pipe::*;
pub use unary::*;
pub use value::*;

use super::{Result, Span};
use crate::Scope;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(ValueExpr),
    Ident(IdentExpr),
    Member(MemberExpr),
    Index(IndexExpr),
    Call(CallExpr),
    Pipe(PipeExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Array(ArrayExpr),
    Object(ObjectExpr),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Value(e) => e.span,
            Self::Ident(e) => e.span,
            Self::Member(e) => e.span,
            Self::Index(e) => e.span,
            Self::Call(e) => e.span,
            Self::Pipe(e) => e.span,
            Self::Binary(e) => e.span,
            Self::Unary(e) => e.span,
            Self::Array(e) => e.span,
            Self::Object(e) => e.span,
        }
    }

    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        match self {
            Self::Value(e) => e.eval(scope),
            Self::Ident(e) => e.eval(scope),
            Self::Member(e) => e.eval(scope),
            Self::Index(e) => e.eval(scope),
            Self::Call(e) => e.eval(scope),
            Self::Pipe(e) => e.eval(scope),
            Self::Binary(e) => e.eval(scope),
            Self::Unary(e) => e.eval(scope),
            Self::Array(e) => e.eval(scope),
            Self::Object(e) => e.eval(scope),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOp, EvalError, UnaryOp};

    fn span() -> Span {
        Span::new(0, 1)
    }

    fn val_expr(v: xval::Value) -> Expr {
        Expr::Value(ValueExpr {
            value: v,
            span: span(),
        })
    }

    fn ident_expr(name: &str) -> Expr {
        Expr::Ident(IdentExpr {
            name: name.to_string(),
            span: span(),
        })
    }

    fn binary_expr(left: Expr, op: BinaryOp, right: Expr) -> Expr {
        Expr::Binary(BinaryExpr {
            left: Box::new(left),
            op,
            right: Box::new(right),
            span: span(),
        })
    }

    #[test]
    fn eval_literal_null() {
        let ctx = Scope::new();
        let result = val_expr(xval::Value::Null).eval(&ctx).unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn eval_literal_bool() {
        let ctx = Scope::new();
        let result = val_expr(xval::Value::from_bool(true)).eval(&ctx).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn eval_literal_int() {
        let ctx = Scope::new();
        let result = val_expr(xval::Value::from_i64(42)).eval(&ctx).unwrap();
        assert_eq!(result, 42i64);
    }

    #[test]
    fn eval_literal_float() {
        let ctx = Scope::new();
        let result = val_expr(xval::Value::from_f64(3.14)).eval(&ctx).unwrap();
        assert_eq!(result, 3.14f64);
    }

    #[test]
    fn eval_literal_string() {
        let ctx = Scope::new();
        let result = val_expr(xval::Value::from_str("hello")).eval(&ctx).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn eval_ident_found() {
        let mut ctx = Scope::new();
        ctx.set_var("x", xval::Value::from_i64(10));
        let result = ident_expr("x").eval(&ctx).unwrap();
        assert_eq!(result, 10i64);
    }

    #[test]
    fn eval_ident_undefined() {
        let ctx = Scope::new();
        let err = ident_expr("x").eval(&ctx).unwrap_err();
        assert!(matches!(err, EvalError::UndefinedVariable(_)));
    }

    #[test]
    fn eval_add_ints() {
        let ctx = Scope::new();
        let expr = binary_expr(
            val_expr(xval::Value::from_i64(2)),
            BinaryOp::Add,
            val_expr(xval::Value::from_i64(3)),
        );
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, 5i64);
    }

    #[test]
    fn eval_float_promotion() {
        let ctx = Scope::new();
        let expr = binary_expr(
            val_expr(xval::Value::from_i64(1)),
            BinaryOp::Add,
            val_expr(xval::Value::from_f64(2.5)),
        );
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, 3.5f64);
    }

    #[test]
    fn eval_division_by_zero() {
        let ctx = Scope::new();
        let expr = binary_expr(
            val_expr(xval::Value::from_i64(10)),
            BinaryOp::Div,
            val_expr(xval::Value::from_i64(0)),
        );
        let err = expr.eval(&ctx).unwrap_err();
        assert!(matches!(err, EvalError::DivisionByZero(_)));
    }

    #[test]
    fn eval_string_concat() {
        let ctx = Scope::new();
        let expr = binary_expr(
            val_expr(xval::Value::from_str("hello")),
            BinaryOp::Add,
            val_expr(xval::Value::from_str(" world")),
        );
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn eval_comparison() {
        let ctx = Scope::new();
        let expr = binary_expr(
            val_expr(xval::Value::from_i64(1)),
            BinaryOp::Lt,
            val_expr(xval::Value::from_i64(2)),
        );
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn eval_equality() {
        let ctx = Scope::new();
        let expr = binary_expr(
            val_expr(xval::Value::from_str("a")),
            BinaryOp::Eq,
            val_expr(xval::Value::from_str("a")),
        );
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn eval_logical_and_short_circuit() {
        let ctx = Scope::new();
        let expr = binary_expr(
            val_expr(xval::Value::from_bool(false)),
            BinaryOp::And,
            ident_expr("missing"),
        );
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn eval_logical_or_short_circuit() {
        let ctx = Scope::new();
        let expr = binary_expr(
            val_expr(xval::Value::from_bool(true)),
            BinaryOp::Or,
            ident_expr("missing"),
        );
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn eval_unary_not() {
        let ctx = Scope::new();
        let expr = Expr::Unary(UnaryExpr {
            op: UnaryOp::Not,
            operand: Box::new(val_expr(xval::Value::from_bool(true))),
            span: span(),
        });
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn eval_unary_neg() {
        let ctx = Scope::new();
        let expr = Expr::Unary(UnaryExpr {
            op: UnaryOp::Neg,
            operand: Box::new(val_expr(xval::Value::from_i64(5))),
            span: span(),
        });
        let result = expr.eval(&ctx).unwrap();
        assert_eq!(result, -5i64);
    }

    #[test]
    fn eval_array_literal() {
        let ctx = Scope::new();
        let expr = Expr::Array(ArrayExpr {
            elements: vec![
                val_expr(xval::Value::from_i64(1)),
                val_expr(xval::Value::from_i64(2)),
            ],
            span: span(),
        });
        let result = expr.eval(&ctx).unwrap();
        assert!(result.is_array());
        assert_eq!(result.as_array().len(), 2);
    }

    #[test]
    fn eval_object_literal() {
        let ctx = Scope::new();
        let expr = Expr::Object(ObjectExpr {
            entries: vec![
                ("a".into(), val_expr(xval::Value::from_i64(1))),
                ("b".into(), val_expr(xval::Value::from_str("two"))),
            ],
            span: span(),
        });
        let result = expr.eval(&ctx).unwrap();
        assert!(result.is_struct());
        assert_eq!(result.as_struct().len(), 2);
    }
}
