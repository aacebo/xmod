#[derive(Debug, Clone, PartialEq)]
pub struct NotIterableError;
impl std::fmt::Display for NotIterableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value is not iterable")
    }
}
