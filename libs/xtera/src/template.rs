use std::sync::Arc;

use crate::{Scope, ast, parse};

/// A parsed template — a block of nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    src: Arc<str>,
    block: ast::BlockNode,
}

impl Template {
    pub fn new(src: Arc<str>, block: ast::BlockNode) -> Self {
        Self { src, block }
    }

    pub fn parse(source: &str) -> parse::Result<Self> {
        parse::parse(source)
    }

    pub fn src(&self) -> &str {
        &self.src
    }

    pub fn nodes(&self) -> &[ast::Node] {
        &self.block.nodes
    }

    pub fn span(&self) -> &ast::Span {
        &self.block.span
    }

    pub fn render(&self, scope: &Scope) -> ast::Result<String> {
        self.block.render(scope)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Template {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.src)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Template {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Template::parse(&s).map_err(serde::de::Error::custom)
    }
}
