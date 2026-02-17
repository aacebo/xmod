use crate::{Context, ValidError, Validate, rules::Rule};

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct AnySchema(Vec<Rule>);

impl AnySchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn exists(&self, name: &str) -> bool {
        self.0.iter().find(|r| r.name() == name).is_some()
    }

    pub fn get(&self, name: &str) -> Option<&Rule> {
        self.0.iter().find(|r| r.name() == name)
    }

    pub fn register(&mut self, rule: Rule) -> &mut Self {
        self.0.push(rule);
        self
    }

    pub fn rule(mut self, rule: Rule) -> Self {
        self.register(rule);
        self
    }

    pub fn validate(&self, value: &xval::Value) -> Result<xval::Value, ValidError> {
        Validate::validate(
            self,
            &Context {
                rule: "type::any".to_string(),
                path: xpath::Path::default(),
                value: value.clone(),
            },
        )
    }
}

impl std::fmt::Display for AnySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Validate for AnySchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let mut next = ctx.clone();
        let mut error = ValidError::new(&ctx.rule, ctx.path.clone()).build();

        for rule in &self.0 {
            next.rule = rule.name().to_string();
            next.value = match rule.validate(&next) {
                Ok(v) => v,
                Err(err) => {
                    error.errors.push(err);
                    continue;
                }
            };
        }

        if !error.errors.is_empty() {
            return Err(error);
        }

        Ok(next.value)
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
        assert_eq!(err.errors[0].rule, "one_of");
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
        // Vec preserves insertion order: "required" before "one_of"
        assert_eq!(err.errors.len(), 2);
        assert_eq!(err.errors[0].rule, "required");
        assert_eq!(err.errors[1].rule, "one_of");
    }
}
