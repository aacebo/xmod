use crate::ValidError;

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub name: String,
    pub path: xpath::Path,
    pub value: xval::Value,
}

impl Context {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_path(mut self, path: xpath::Path) -> Self {
        self.path = path;
        self
    }

    pub fn with_value(mut self, value: xval::Value) -> Self {
        self.value = value;
        self
    }

    pub fn error(&self, message: &str) -> ValidError {
        ValidError::new(self.path.clone())
            .name(&self.name)
            .message(message)
            .build()
    }
}

impl<T: xval::ToValue> From<T> for Context {
    fn from(value: T) -> Self {
        Self {
            value: value.to_value(),
            ..Self::default()
        }
    }
}
