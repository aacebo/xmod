use xok::XError;

#[derive(Debug, Clone)]
pub struct FluxError {
    message: String,
}

impl FluxError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for FluxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FluxError {}

impl XError for FluxError {
    fn name(&self) -> &'static str {
        "FluxError"
    }

    fn module(&self) -> &'static str {
        module_path!()
    }
}
