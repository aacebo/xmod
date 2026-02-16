use crate::ast::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidIndexError {
    pub span: Span,
}

impl std::fmt::Display for InvalidIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index expression must evaluate to an integer")
    }
}
