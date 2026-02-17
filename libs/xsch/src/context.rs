use crate::ValidError;

#[derive(Debug, Default, Clone)]
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

impl From<xval::Value> for Context {
    fn from(value: xval::Value) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }
}
