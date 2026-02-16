use crate::Scope;
use crate::ast::{Expr, Result, Span};

use super::{Node, render_nodes};

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchNode {
    pub expr: Expr,
    pub cases: Vec<SwitchCase>,
    pub default: Option<Vec<Node>>,
    pub span: Span,
}

impl SwitchNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let expr_val = self.expr.eval(scope)?;

        for case in &self.cases {
            let case_val = case.value.eval(scope)?;
            if expr_val == case_val {
                return render_nodes(&case.body, scope);
            }
        }

        if let Some(default_body) = &self.default {
            return render_nodes(default_body, scope);
        }

        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub value: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}
