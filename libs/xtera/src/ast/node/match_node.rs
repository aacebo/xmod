use crate::Scope;
use crate::ast::{Expr, Result, Span};

use super::BlockNode;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchNode {
    pub expr: Expr,
    pub arms: Vec<MatchNodeArm>,
    pub default: Option<BlockNode>,
    pub span: Span,
}

impl MatchNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let expr_val = self.expr.eval(scope)?;

        for arm in &self.arms {
            let pattern_val = arm.pattern.eval(scope)?;
            if expr_val == pattern_val {
                return arm.body.render(scope);
            }
        }

        if let Some(default_body) = &self.default {
            return default_body.render(scope);
        }

        Ok(String::new())
    }
}

impl std::fmt::Display for MatchNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchNodeArm {
    pub pattern: Expr,
    pub body: BlockNode,
    pub span: Span,
}
