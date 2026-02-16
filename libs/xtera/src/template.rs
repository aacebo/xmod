use crate::{ast, eval, parse};

/// A parsed template — a sequence of nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    pub(crate) nodes: Vec<ast::Node>,
    pub(crate) span: ast::Span,
}

impl Template {
    pub(crate) fn new(nodes: Vec<ast::Node>, span: ast::Span) -> Self {
        Self { nodes, span }
    }

    pub fn parse(src: &str) -> parse::Result<Self> {
        parse::parse(src)
    }

    pub fn render(&self, ctx: &mut eval::Context) -> eval::Result<String> {
        eval::render(&self.nodes, ctx)
    }
}
