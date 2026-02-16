use crate::Scope;
use crate::ast::{Expr, Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpr {
    pub expr: Box<Expr>,
    pub arms: Vec<MatchArm>,
    pub default: Option<Box<Expr>>,
    pub span: Span,
}

impl MatchExpr {
    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        let expr_val = self.expr.eval(scope)?;

        for arm in &self.arms {
            let pattern_val = arm.pattern.eval(scope)?;

            if expr_val == pattern_val {
                return arm.value.eval(scope);
            }
        }

        if let Some(default) = &self.default {
            return default.eval(scope);
        }

        Ok(xval::Value::Null)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Expr,
    pub value: Expr,
    pub span: Span,
}
