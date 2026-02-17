#[derive(Debug, Clone, PartialEq)]
pub struct ValidError {
    pub rule: String,            // "min"
    pub path: xpath::Path,       // "test[1].name"
    pub message: Option<String>, // "length must be at least 1"
    pub errors: Vec<ValidError>,
}

impl ValidError {
    pub fn new(rule: &str, path: xpath::Path) -> ValidErrorBuilder {
        ValidErrorBuilder::new(rule, path)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidErrorBuilder {
    rule: String,
    path: xpath::Path,
    message: Option<String>,
    errors: Vec<ValidError>,
}

impl ValidErrorBuilder {
    pub fn new(rule: &str, path: xpath::Path) -> Self {
        Self {
            rule: rule.to_string(),
            path,
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
