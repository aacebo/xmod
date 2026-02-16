#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl std::error::Error for ParseError {}

impl xok::XError for ParseError {
    fn name(&self) -> &'static str {
        "ParseError"
    }

    fn module(&self) -> &'static str {
        "xpath"
    }
}
