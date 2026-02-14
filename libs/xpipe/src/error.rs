use xok::XError;

pub type Result<T> = std::result::Result<T, TaskError>;

#[derive(Debug, Clone)]
pub struct TaskError {
    message: String,
}

impl TaskError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TaskError {}

impl XError for TaskError {
    fn name(&self) -> &'static str {
        "TaskError"
    }

    fn module(&self) -> &'static str {
        module_path!()
    }
}
