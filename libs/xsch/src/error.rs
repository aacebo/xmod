#[derive(Debug, Clone, PartialEq)]
pub struct ValidError {
    pub rule: String,            // "min"
    pub path: String,            // "test[1].name"
    pub message: Option<String>, // "length must be at least 1"
    pub errors: Vec<ValidError>,
}

impl ValidError {
    pub fn new(rule: &str, path: &str) -> ValidErrorBuilder {
        ValidErrorBuilder::new(rule, path)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidErrorBuilder {
    rule: String,
    path: String,
    message: Option<String>,
    errors: Vec<ValidError>,
}

impl ValidErrorBuilder {
    pub fn new(rule: &str, path: &str) -> Self {
        Self {
            rule: rule.to_string(),
            path: path.to_string(),
            message: None,
            errors: vec![],
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn build(self) -> ValidError {
        ValidError {
            rule: self.rule,
            path: self.path,
            message: self.message,
            errors: self.errors,
        }
    }
}
