#[derive(Debug, Clone, PartialEq)]
pub struct DivisionByZeroError;
impl std::fmt::Display for DivisionByZeroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "division by zero")
    }
}
