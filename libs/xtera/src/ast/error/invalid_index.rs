#[derive(Debug, Clone, PartialEq)]
pub struct InvalidIndexError;
impl std::fmt::Display for InvalidIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index expression must evaluate to an integer")
    }
}
