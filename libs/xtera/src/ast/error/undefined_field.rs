#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedFieldError {
    pub name: String,
}

impl std::fmt::Display for UndefinedFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "undefined field '{}'", self.name)
    }
}
