use crate::Scope;
use crate::ast::{Result, Span};

use super::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode {
    pub nodes: Vec<Node>,
    pub span: Span,
}

impl BlockNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let mut output = String::new();

        for node in &self.nodes {
            output.push_str(&node.render(scope)?);
        }

        Ok(output)
    }
}

impl std::fmt::Display for BlockNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
