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
