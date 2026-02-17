use crate::ValidError;

#[derive(Debug, Clone)]
pub struct Context {
    pub rule: String,
    pub path: xpath::Path,
    pub value: xval::Value,
}

impl Context {
    pub fn error(&self, message: &str) -> ValidError {
        ValidError::new(&self.rule, self.path.clone())
            .message(message)
            .build()
    }
}
