use crate::ast::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct DivisionByZeroError {
    pub span: Span,
}

impl std::fmt::Display for DivisionByZeroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero")
    }
}
