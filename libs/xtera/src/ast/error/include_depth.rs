#[derive(Debug, Clone, PartialEq)]
pub struct IncludeDepthError;
impl std::fmt::Display for IncludeDepthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "maximum include depth exceeded")
    }
}
