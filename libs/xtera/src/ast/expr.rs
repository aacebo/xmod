use std::collections::HashMap;

use super::eval::{
    EvalError, EvalErrorKind, Result, expect_number, is_truthy, value_to_usize, value_type_name,
};
use super::{BinaryOp, Span, UnaryOp};
use crate::Scope;

// ── Expr enum ───────────────────────────────────────────────────────

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

// ── ValueExpr ───────────────────────────────────────────────────────

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

// ── IdentExpr ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr {
    pub name: String,
    pub span: Span,
}

impl IdentExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        scope.var(&self.name).cloned().ok_or_else(|| {
            EvalError::new(
                EvalErrorKind::UndefinedVariable(self.name.clone()),
                self.span,
            )
        })
    }
}

// ── MemberExpr ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpr {
    pub object: Box<Expr>,
    pub field: String,
    pub span: Span,
}

impl MemberExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let obj = self.object.eval(scope)?;
        if !obj.is_struct() {
            return Err(EvalError::new(
                EvalErrorKind::TypeError {
                    expected: "struct",
                    got: value_type_name(&obj),
                },
                self.span,
            ));
        }

        obj.as_struct()
            .field(xval::Ident::key(&self.field))
            .map(|v| v.as_value())
            .ok_or_else(|| {
                EvalError::new(EvalErrorKind::UndefinedField(self.field.clone()), self.span)
            })
    }
}

// ── IndexExpr ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
    pub span: Span,
}

impl IndexExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let obj = self.object.eval(scope)?;
        let idx = self.index.eval(scope)?;

        if !obj.is_array() {
            return Err(EvalError::new(
                EvalErrorKind::TypeError {
                    expected: "array",
                    got: value_type_name(&obj),
                },
                self.span,
            ));
        }

        let i = value_to_usize(&idx, self.span)?;
        let arr = obj.as_array();
        arr.index(i).map(|v| v.as_value()).ok_or_else(|| {
            EvalError::new(
                EvalErrorKind::IndexOutOfBounds {
                    index: i,
                    len: arr.len(),
                },
                self.span,
            )
        })
    }
}

// ── CallExpr ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl CallExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let name = match &*self.callee {
            Expr::Ident(ident) => ident.name.as_str(),
            _ => return Err(EvalError::new(EvalErrorKind::NotCallable, self.span)),
        };

        let func = scope
            .func(name)
            .ok_or_else(|| EvalError::new(EvalErrorKind::NotCallable, self.span))?;

        let evaluated_args: Vec<xval::Value> = self
            .args
            .iter()
            .map(|a| a.eval(scope))
            .collect::<Result<_>>()?;

        func.invoke(&evaluated_args)
    }
}

// ── PipeExpr ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct PipeExpr {
    pub value: Box<Expr>,
    pub name: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl PipeExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let val = self.value.eval(scope)?;
        let evaluated_args: Vec<xval::Value> = self
            .args
            .iter()
            .map(|a| a.eval(scope))
            .collect::<Result<_>>()?;

        let pipe = scope.pipe(&self.name).ok_or_else(|| {
            EvalError::new(EvalErrorKind::UndefinedPipe(self.name.clone()), self.span)
        })?;

        pipe.invoke(&val, &evaluated_args)
    }
}

// ── BinaryExpr ──────────────────────────────────────────────────────

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

        let li = l.to_i64();
        let ri = r.to_i64();

        match op {
            BinaryOp::Add => Ok(xval::Value::from_i64(li.wrapping_add(ri))),
            BinaryOp::Sub => Ok(xval::Value::from_i64(li.wrapping_sub(ri))),
            BinaryOp::Mul => Ok(xval::Value::from_i64(li.wrapping_mul(ri))),
            BinaryOp::Div => {
                if ri == 0 {
                    return Err(EvalError::new(EvalErrorKind::DivisionByZero, span));
                }
                Ok(xval::Value::from_i64(li.wrapping_div(ri)))
            }
            BinaryOp::Mod => {
                if ri == 0 {
                    return Err(EvalError::new(EvalErrorKind::DivisionByZero, span));
                }
                Ok(xval::Value::from_i64(li.wrapping_rem(ri)))
            }
            _ => unreachable!(),
        }
    }
}

// ── UnaryExpr ───────────────────────────────────────────────────────

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
            UnaryOp::Not => Ok(xval::Value::from_bool(!is_truthy(&val))),
            UnaryOp::Neg => {
                let n = expect_number(&val, self.span)?;
                if n.is_float() {
                    Ok(xval::Value::from_f64(-n.to_f64()))
                } else {
                    Ok(xval::Value::from_i64(n.to_i64().wrapping_neg()))
                }
            }
        }
    }
}

// ── ArrayExpr ───────────────────────────────────────────────────────

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

// ── ObjectExpr ──────────────────────────────────────────────────────

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

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(err.kind, EvalErrorKind::UndefinedVariable("x".into()));
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
        assert_eq!(err.kind, EvalErrorKind::DivisionByZero);
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
