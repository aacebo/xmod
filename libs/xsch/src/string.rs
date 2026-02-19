use xval::ToValue;

use crate::{
    Context, Equals, Max, Min, Options, Required, RuleSet, Schema, ToSchema, ValidError, Validator,
};

pub fn string() -> StringSchema {
    StringSchema::default()
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct StringSchema(pub(crate) RuleSet);

impl StringSchema {
    pub fn equals(mut self, value: &str) -> Self {
        self.0 = self.0.add(Equals::from(value.to_value()).into());
        self
    }

    pub fn options(mut self, options: &[&str]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.to_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }

    pub fn min(mut self, min: usize) -> Self {
        self.0 = self.0.add(Min::from(xval::Number::from_usize(min)).into());
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.0 = self.0.add(Max::from(xval::Number::from_usize(max)).into());
        self
    }

    #[cfg(feature = "regex")]
    pub fn pattern(mut self, pattern: &str) -> Self {
        use crate::Pattern;

        self.0 = self.0.add(Pattern::from(pattern.to_string()).into());
        self
    }
}

impl ToSchema for StringSchema {
    fn to_schema(&self) -> Schema {
        Schema::String(self.clone())
    }
}

impl From<StringSchema> for Schema {
    fn from(value: StringSchema) -> Self {
        Self::String(value)
    }
}

impl Validator for StringSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let value = self.0.validate(ctx)?;

        if !value.is_null() && !value.is_string() {
            return Err(ctx.error("expected string"));
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_string() {
        let schema = string();
        assert!(schema.validate(&"hello world".to_value().into()).is_ok());
    }

    #[test]
    fn validate_rejects_non_string() {
        let schema = string();
        let err = schema.validate(&42i32.to_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected string"));
    }

    #[test]
    fn validate_null_passes_without_required() {
        let schema = string();
        assert!(schema.validate(&xval::valueof!(null).into()).is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = string().required();
        let err = schema.validate(&xval::valueof!(null).into()).unwrap_err();
        assert_eq!(err.errors[0].message.as_deref(), Some("required"));
    }

    #[test]
    fn validate_equals() {
        let schema = string().equals("hello, world!");
        assert!(schema.validate(&"hello, world!".to_value().into()).is_ok());
        assert!(schema.validate(&"hello world!".to_value().into()).is_err());
    }

    #[test]
    fn validate_options() {
        let schema = string().options(&["a", "b", "c"]);
        assert!(schema.validate(&"a".to_value().into()).is_ok());
        assert!(schema.validate(&"d".to_value().into()).is_err());
    }

    #[test]
    fn validate_required_and_equals() {
        let schema = string().required().equals("sun");
        assert!(schema.validate(&"sun".to_value().into()).is_ok());
        assert!(schema.validate(&"moon".to_value().into()).is_err());
        assert!(schema.validate(&xval::valueof!(null).into()).is_err());
    }

    #[test]
    fn validate_min() {
        let schema = string().min(3);
        assert!(schema.validate(&"hi".to_value().into()).is_err());
        assert!(schema.validate(&"hel".to_value().into()).is_ok());
        assert!(schema.validate(&"hello".to_value().into()).is_ok());
    }

    #[test]
    fn validate_max() {
        let schema = string().max(3);
        assert!(schema.validate(&"hi".to_value().into()).is_ok());
        assert!(schema.validate(&"hel".to_value().into()).is_ok());
        assert!(schema.validate(&"hello".to_value().into()).is_err());
    }

    #[cfg(feature = "regex")]
    #[test]
    fn validate_pattern() {
        let schema = string().pattern("Homer (.)\\. Simpson");
        assert!(
            schema
                .validate(&"Homer J. Simpson".to_value().into())
                .is_ok()
        );
        assert!(schema.validate(&"Sam Simpson".to_value().into()).is_err());
    }
}
