#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedVariableError {
    pub name: String,
}

impl std::fmt::Display for UndefinedVariableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "undefined variable '{}'", self.name)
    }
}
