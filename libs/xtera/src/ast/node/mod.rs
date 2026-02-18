mod block;
mod for_node;
mod if_node;
mod include;
mod interp;
mod match_node;
mod text;

pub use block::*;
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
    Block(BlockNode),
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Self::Text(n) => n.span.clone(),
            Self::Interp(n) => n.span.clone(),
            Self::If(n) => n.span.clone(),
            Self::For(n) => n.span.clone(),
            Self::Match(n) => n.span.clone(),
            Self::Include(n) => n.span.clone(),
            Self::Block(n) => n.span.clone(),
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
            Self::Block(n) => n.render(scope),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}
