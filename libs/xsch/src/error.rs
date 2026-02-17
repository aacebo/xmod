#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    mod serde {
        use crate::*;

        #[test]
        fn serialize() {
            let err = ValidErrorBuilder::new(xpath::Path::parse("a/b").unwrap())
                .rule("required")
                .message("field is required")
                .build();
            let json = serde_json::to_string(&err).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["rule"], "required");
            assert_eq!(v["path"], "a/b");
            assert_eq!(v["message"], "field is required");
        }

        #[test]
        fn roundtrip() {
            let err = ValidErrorBuilder::new(xpath::Path::parse("x/0").unwrap())
                .rule("equals")
                .message("values differ")
                .build();
            let json = serde_json::to_string(&err).unwrap();
            let restored: ValidError = serde_json::from_str(&json).unwrap();
            assert_eq!(err, restored);
        }
    }
}
