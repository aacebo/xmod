use crate::{Scope, ast, parse};

/// A parsed template — a block of nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    block: ast::BlockNode,
}

impl Template {
    pub(crate) fn new(block: ast::BlockNode) -> Self {
        Self { block }
    }

    pub fn nodes(&self) -> &[ast::Node] {
        &self.block.nodes
    }

    pub fn span(&self) -> &ast::Span {
        &self.block.span
    }

    pub fn parse(src: &str) -> parse::Result<Self> {
        parse::parse(src)
    }

    pub fn render(&self, scope: &Scope) -> ast::Result<String> {
        self.block.render(scope)
    }
}
