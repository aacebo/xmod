use crate::ast::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct NotCallableError {
    pub span: Span,
}

impl std::fmt::Display for NotCallableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value is not callable")
    }
}
