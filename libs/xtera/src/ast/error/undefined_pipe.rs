#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedPipeError {
    pub name: String,
}

impl std::fmt::Display for UndefinedPipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "undefined pipe '{}'", self.name)
    }
}
