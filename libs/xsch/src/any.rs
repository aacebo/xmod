use crate::{Context, ValidError, Validate, rules::RuleRegistry};

#[derive(Debug, Default, Clone)]
pub struct AnySchema(RuleRegistry);

impl AnySchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rule<Rule: Validate + 'static>(mut self, name: &str, rule: Rule) -> Self {
        self.0.register(name, rule);
        self
    }

    pub fn validate(&self, value: &xval::Value) -> Result<xval::Value, ValidError> {
        self.0.validate(&Context {
            rule: "type::any".to_string(),
            path: xpath::Path::default(),
            value: value.clone(),
        })
    }
}

impl Validate for AnySchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        self.0.validate(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_passes_when_non_null() {
        let schema = AnySchema::new().required();
        let result = schema.validate(&xval::Value::from_str("hello"));
        assert!(result.is_ok());
    }

    #[test]
    fn required_fails_when_null() {
        let schema = AnySchema::new().required();
        let result = schema.validate(&xval::Value::Null);
        let err = result.unwrap_err();
        assert_eq!(err.rule, "type::any");
        assert_eq!(err.errors.len(), 1);
        assert_eq!(err.errors[0].rule, "required");
    }

    #[test]
    fn one_of_passes_when_value_matches() {
        let options = vec![
            xval::Value::from_str("a"),
            xval::Value::from_str("b"),
            xval::Value::from_str("c"),
        ];
        let schema = AnySchema::new().options(&options);
        let result = schema.validate(&xval::Value::from_str("b"));
        assert!(result.is_ok());
    }

    #[test]
    fn one_of_fails_when_value_not_in_options() {
        let options = vec![
            xval::Value::from_str("a"),
            xval::Value::from_str("b"),
            xval::Value::from_str("c"),
        ];
        let schema = AnySchema::new().options(&options);
        let result = schema.validate(&xval::Value::from_str("d"));
        let err = result.unwrap_err();
        assert_eq!(err.rule, "type::any");
        assert_eq!(err.errors.len(), 1);
        assert_eq!(err.errors[0].rule, "one-of");
    }

    #[test]
    fn required_and_one_of_passes() {
        let options = vec![xval::Value::from_str("a"), xval::Value::from_str("b")];
        let schema = AnySchema::new().required().options(&options);
        let result = schema.validate(&xval::Value::from_str("a"));
        assert!(result.is_ok());
    }

    #[test]
    fn required_and_one_of_fails_on_null() {
        let options = vec![xval::Value::from_str("a"), xval::Value::from_str("b")];
        let schema = AnySchema::new().required().options(&options);
        let result = schema.validate(&xval::Value::Null);
        let err = result.unwrap_err();
        assert_eq!(err.rule, "type::any");
        // BTreeMap iterates alphabetically: "one-of" before "required"
        assert_eq!(err.errors.len(), 2);
        assert_eq!(err.errors[0].rule, "one-of");
        assert_eq!(err.errors[1].rule, "required");
    }
}
