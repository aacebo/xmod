use crate::ast::Span;

pub type Result<T> = std::result::Result<T, EvalError>;

#[derive(Debug, Clone, PartialEq)]
pub struct EvalError {
    pub kind: EvalErrorKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalErrorKind {
    /// Variable not found in context.
    UndefinedVariable(String),
    /// Pipe function not registered.
    UndefinedPipe(String),
    /// Field not found on a struct value.
    UndefinedField(String),
    /// Index out of bounds on an array.
    IndexOutOfBounds { index: usize, len: usize },
    /// Type mismatch.
    TypeError { expected: &'static str, got: String },
    /// Division or modulo by zero.
    DivisionByZero,
    /// Value is not callable.
    NotCallable,
    /// Value is not iterable (used in @for).
    NotIterable,
    /// Index expression did not evaluate to an integer.
    InvalidIndex,
    /// Integer overflow during arithmetic.
    Overflow,
}

impl EvalError {
    pub fn new(kind: EvalErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "eval error at {}..{}: {}",
            self.span.start, self.span.end, self.kind
        )
    }
}

impl std::fmt::Display for EvalErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedVariable(name) => write!(f, "undefined variable '{name}'"),
            Self::UndefinedPipe(name) => write!(f, "undefined pipe '{name}'"),
            Self::UndefinedField(name) => write!(f, "undefined field '{name}'"),
            Self::IndexOutOfBounds { index, len } => {
                write!(f, "index {index} out of bounds (len {len})")
            }
            Self::TypeError { expected, got } => write!(f, "expected {expected}, got {got}"),
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::NotCallable => write!(f, "value is not callable"),
            Self::NotIterable => write!(f, "value is not iterable"),
            Self::InvalidIndex => write!(f, "index expression must evaluate to an integer"),
            Self::Overflow => write!(f, "integer overflow"),
        }
    }
}

impl std::error::Error for EvalError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_undefined_variable() {
        let err = EvalError::new(
            EvalErrorKind::UndefinedVariable("x".into()),
            Span::new(0, 1),
        );
        assert_eq!(
            err.to_string(),
            "eval error at 0..1: undefined variable 'x'"
        );
    }

    #[test]
    fn display_type_error() {
        let err = EvalError::new(
            EvalErrorKind::TypeError {
                expected: "number",
                got: "string".into(),
            },
            Span::new(5, 10),
        );
        assert_eq!(
            err.to_string(),
            "eval error at 5..10: expected number, got string"
        );
    }

    #[test]
    fn display_division_by_zero() {
        let err = EvalError::new(EvalErrorKind::DivisionByZero, Span::new(3, 8));
        assert_eq!(err.to_string(), "eval error at 3..8: division by zero");
    }

    #[test]
    fn display_index_out_of_bounds() {
        let err = EvalError::new(
            EvalErrorKind::IndexOutOfBounds { index: 5, len: 3 },
            Span::new(0, 4),
        );
        assert_eq!(
            err.to_string(),
            "eval error at 0..4: index 5 out of bounds (len 3)"
        );
    }
}
