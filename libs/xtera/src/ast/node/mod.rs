mod for_node;
mod if_node;
mod include;
mod interp;
mod match_node;
mod text;

pub use for_node::*;
pub use if_node::*;
pub use include::*;
pub use interp::*;
pub use match_node::*;
pub use text::*;

use super::{Result, Span};
use crate::Scope;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Text(TextNode),
    Interp(InterpNode),
    If(IfNode),
    For(ForNode),
    Match(MatchNode),
    Include(IncludeNode),
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Self::Text(n) => n.span,
            Self::Interp(n) => n.span,
            Self::If(n) => n.span,
            Self::For(n) => n.span,
            Self::Match(n) => n.span,
            Self::Include(n) => n.span,
        }
    }

    pub fn render(&self, scope: &Scope) -> Result<String> {
        match self {
            Self::Text(n) => n.render(scope),
            Self::Interp(n) => n.render(scope),
            Self::If(n) => n.render(scope),
            Self::For(n) => n.render(scope),
            Self::Match(n) => n.render(scope),
            Self::Include(n) => n.render(scope),
        }
    }
}

pub fn render_nodes(nodes: &[Node], scope: &Scope) -> Result<String> {
    let mut output = String::new();
    for node in nodes {
        output.push_str(&node.render(scope)?);
    }
    Ok(output)
}
