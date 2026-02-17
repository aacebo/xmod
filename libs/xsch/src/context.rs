use crate::{ValidError, ValidErrorBuilder};

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub rule: Option<String>,
    pub path: xpath::Path,
    pub value: xval::Value,
}

impl Context {
    pub fn error(&self, message: &str) -> ValidError {
        let mut builder = ValidErrorBuilder::new(self.path.clone()).message(message);

        if let Some(rule) = &self.rule {
            builder = builder.rule(&rule);
        }

        builder.build()
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
