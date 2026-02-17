#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ValidError {
    pub rule: Option<String>,    // "min"
    pub path: xpath::Path,       // "test[1].name"
    pub message: Option<String>, // "length must be at least 1"
    pub errors: Vec<ValidError>,
}

impl ValidError {
    pub fn new(path: xpath::Path) -> ValidErrorBuilder {
        ValidErrorBuilder::new(path)
    }
}

impl std::fmt::Display for ValidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")?;

        if let Some(rule) = &self.rule {
            write!(f, " [{}]", &rule)?;
        }

        writeln!(f, "@ {}", &self.path)?;

        if let Some(message) = &self.message {
            writeln!(f, "  {}", &message)?;
        }

        for err in &self.errors {
            write!(f, "{}", err)?;
        }

        Ok(())
    }
}

impl std::error::Error for ValidError {}

impl xok::XError for ValidError {
    fn name(&self) -> &'static str {
        "ValidError"
    }

    fn module(&self) -> &'static str {
        module_path!()
    }
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
    use crate::*;

    mod display {
        use super::*;

        #[test]
        fn with_rule_and_message() {
            let err = ValidError::new(xpath::Path::parse("a/b").unwrap())
                .rule("required")
                .message("field is required")
                .build();
            let output = err.to_string();
            assert!(output.contains("Error [required]"));
            assert!(output.contains("@ a/b"));
            assert!(output.contains("  field is required"));
        }

        #[test]
        fn without_rule() {
            let err = ValidError::new(xpath::Path::parse("a").unwrap())
                .message("something went wrong")
                .build();
            let output = err.to_string();
            assert!(output.contains("Error@ a"));
            assert!(output.contains("  something went wrong"));
            assert!(!output.contains("["));
        }

        #[test]
        fn without_message() {
            let err = ValidError::new(xpath::Path::parse("x").unwrap())
                .rule("equals")
                .build();
            let output = err.to_string();
            assert!(output.contains("Error [equals]"));
            assert!(output.contains("@ x"));
            assert_eq!(output.lines().count(), 1);
        }

        #[test]
        fn nested_errors() {
            let child1 = ValidError::new(xpath::Path::parse("a").unwrap())
                .rule("required")
                .message("required")
                .build();
            let child2 = ValidError::new(xpath::Path::parse("a").unwrap())
                .rule("equals")
                .message("not equal")
                .build();
            let mut parent = ValidError::new(xpath::Path::parse("a").unwrap()).build();
            parent.errors.push(child1);
            parent.errors.push(child2);

            let output = parent.to_string();
            assert!(output.contains("Error [required]"));
            assert!(output.contains("  required"));
            assert!(output.contains("Error [equals]"));
            assert!(output.contains("  not equal"));
        }
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            let err = ValidError::new(xpath::Path::parse("a/b").unwrap())
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
            let err = ValidError::new(xpath::Path::parse("x/0").unwrap())
                .rule("equals")
                .message("values differ")
                .build();
            let json = serde_json::to_string(&err).unwrap();
            let restored: ValidError = serde_json::from_str(&json).unwrap();
            assert_eq!(err, restored);
        }
    }
}
