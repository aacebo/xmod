use crate::Scope;
use crate::ast::{Expr, Result, Span, is_truthy};

use super::BlockNode;

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub branches: Vec<IfBranch>,
    pub else_body: Option<BlockNode>,
    pub span: Span,
}

impl IfNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        for branch in &self.branches {
            let cond = branch.condition.eval(scope)?;

            if is_truthy(&cond) {
                return branch.body.render(scope);
            }
        }

        if let Some(else_body) = &self.else_body {
            return else_body.render(scope);
        }

        Ok(String::new())
    }
}

impl std::fmt::Display for IfNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: BlockNode,
    pub span: Span,
}
