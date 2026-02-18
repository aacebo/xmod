use crate::Scope;
use crate::ast::{Result, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct TextNode {
    pub text: String,
    pub span: Span,
}

impl TextNode {
    pub fn render(&self, _scope: &Scope) -> Result<String> {
        Ok(self.text.clone())
    }
}

impl std::fmt::Display for TextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_text() {
        let scope = Scope::new();
        let node = TextNode {
            text: "hello world".into(),
            span: Span::new(0, 11),
        };
        assert_eq!(node.render(&scope).unwrap(), "hello world");
    }
}
