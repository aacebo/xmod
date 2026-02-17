#[derive(Debug, Clone, PartialEq)]
pub struct ValidError {
    pub rule: Option<String>,    // "min"
    pub path: xpath::Path,       // "test[1].name"
    pub message: Option<String>, // "length must be at least 1"
    pub errors: Vec<ValidError>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidErrorBuilder {
    rule: Option<String>,
    path: xpath::Path,
    message: Option<String>,
    errors: Vec<ValidError>,
}

impl ValidErrorBuilder {
    pub fn new(path: xpath::Path) -> Self {
        Self {
            rule: None,
            path,
            message: None,
            errors: vec![],
        }
    }

    pub fn rule(mut self, name: &str) -> Self {
        self.rule = Some(name.to_string());
        self
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
