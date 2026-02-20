#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedTemplateError {
    pub name: String,
}

impl std::fmt::Display for UndefinedTemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "undefined template '{}'", self.name)
    }
}
