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
