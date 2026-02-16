use crate::{Scope, ast, eval, parse, render};

/// A parsed template — a sequence of nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    nodes: Vec<ast::Node>,
    span: ast::Span,
}

impl Template {
    pub(crate) fn new(nodes: Vec<ast::Node>, span: ast::Span) -> Self {
        Self { nodes, span }
    }

    pub fn nodes(&self) -> &[ast::Node] {
        &self.nodes
    }

    pub fn span(&self) -> &ast::Span {
        &self.span
    }

    pub fn parse(src: &str) -> parse::Result<Self> {
        parse::parse(src)
    }

    pub fn render(&self, scope: &Scope) -> eval::Result<String> {
        render::render(&self.nodes, scope)
    }
}
