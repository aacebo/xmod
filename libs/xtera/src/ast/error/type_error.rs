use crate::ast::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
    pub expected: &'static str,
    pub got: String,
    pub span: Span,
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {}, got {}", self.expected, self.got)
    }
}
