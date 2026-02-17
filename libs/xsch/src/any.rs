use crate::{Context, Equals, Options, Required, RuleSet, Schema, ValidError, Validate};

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct AnySchema(RuleSet);

impl AnySchema {
    pub fn equals(mut self, value: xval::Value) -> Self {
        self.0 = self.0.add(Equals::from(value).into());
        self
    }

    pub fn options(mut self, options: &[xval::Value]) -> Self {
        self.0 = self.0.add(Options::from(options).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }
}

impl From<AnySchema> for Schema {
    fn from(value: AnySchema) -> Self {
        Self::Any(value)
    }
}

impl Validate for AnySchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        self.0.validate(ctx)
    }
}

#[cfg(test)]
mod tests {
    use xval::AsValue;

    use super::*;

    #[test]
    fn validate_any_value() {
        let schema = AnySchema::default();
        let result = schema.validate(&true.as_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_null() {
        let schema = AnySchema::default();
        let result = schema.validate(&xval::Value::Null.into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = AnySchema::default().required();
        let result = schema.validate(&xval::Value::Null.into());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().errors[0].message.as_deref(),
            Some("required")
        );
    }

    #[test]
    fn validate_required_accepts_value() {
        let schema = AnySchema::default().required();
        let result = schema.validate(&42i32.as_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_equals_match() {
        let schema = AnySchema::default().equals("hello".as_value());
        let result = schema.validate(&"hello".as_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_equals_mismatch() {
        let schema = AnySchema::default().equals("hello".as_value());
        let result = schema.validate(&"world".as_value().into());
        assert!(result.is_err());
    }

    #[test]
    fn validate_options_match() {
        let schema =
            AnySchema::default().options(&[1i32.as_value(), "test".as_value(), true.as_value()]);
        let result = schema.validate(&"test".as_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_options_mismatch() {
        let schema = AnySchema::default().options(&[1i32.as_value(), 2i32.as_value()]);
        let result = schema.validate(&3i32.as_value().into());
        assert!(result.is_err());
    }

    #[test]
    fn validate_required_and_options() {
        let schema = AnySchema::default()
            .required()
            .options(&[true.as_value(), false.as_value()]);

        assert!(schema.validate(&true.as_value().into()).is_ok());
        assert!(schema.validate(&false.as_value().into()).is_ok());
        assert!(schema.validate(&xval::Value::Null.into()).is_err());
        assert!(schema.validate(&42i32.as_value().into()).is_err());
    }

    #[test]
    fn validate_collects_multiple_errors() {
        let schema = AnySchema::default().required().equals(true.as_value());
        let err = schema.validate(&xval::Value::Null.into()).unwrap_err();
        assert_eq!(err.errors.len(), 2);
    }
}
