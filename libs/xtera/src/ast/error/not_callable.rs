#[derive(Debug, Clone, PartialEq)]
pub struct NotCallableError;
impl std::fmt::Display for NotCallableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value is not callable")
    }
}
