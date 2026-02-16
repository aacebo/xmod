use crate::Scope;
use crate::ast::{Expr, Result, Span, is_truthy};

use super::{Node, render_nodes};

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub branches: Vec<IfBranch>,
    pub else_body: Option<Vec<Node>>,
    pub span: Span,
}

impl IfNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        for branch in &self.branches {
            let cond = branch.condition.eval(scope)?;
            if is_truthy(&cond) {
                return render_nodes(&branch.body, scope);
            }
        }

        if let Some(else_body) = &self.else_body {
            return render_nodes(else_body, scope);
        }

        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}
