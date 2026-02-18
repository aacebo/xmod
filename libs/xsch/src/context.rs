use crate::ValidError;

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub name: String,
    pub path: xpath::Path,
    pub value: xval::Value,
}

impl Context {
    pub fn error(&self, message: &str) -> ValidError {
        ValidError::new(self.path.clone())
            .name(&self.name)
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
