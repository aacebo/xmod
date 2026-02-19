use crate::{Context, Equals, Options, Required, RuleSet, Schema, ToSchema, ValidError, Validator};

pub fn any() -> AnySchema {
    AnySchema::default()
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct AnySchema(pub(crate) RuleSet);

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

impl ToSchema for AnySchema {
    fn to_schema(&self) -> Schema {
        Schema::Any(self.clone())
    }
}

impl From<AnySchema> for Schema {
    fn from(value: AnySchema) -> Self {
        Self::Any(value)
    }
}

impl Validator for AnySchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        self.0.validate(ctx)
    }
}

#[cfg(test)]
mod tests {
    use xval::ToValue;

    use super::*;

    #[test]
    fn validate_any_value() {
        let schema = any();
        let result = schema.validate(&true.to_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_null() {
        let schema = any();
        let result = schema.validate(&xval::valueof!(null).into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = any().required();
        let result = schema.validate(&xval::valueof!(null).into());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().errors[0].message.as_deref(),
            Some("required")
        );
    }

    #[test]
    fn validate_required_accepts_value() {
        let schema = any().required();
        let result = schema.validate(&42i32.to_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_equals_match() {
        let schema = any().equals("hello".to_value());
        let result = schema.validate(&"hello".to_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_equals_mismatch() {
        let schema = any().equals("hello".to_value());
        let result = schema.validate(&"world".to_value().into());
        assert!(result.is_err());
    }

    #[test]
    fn validate_options_match() {
        let schema = any().options(&[1i32.to_value(), "test".to_value(), true.to_value()]);
        let result = schema.validate(&"test".to_value().into());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_options_mismatch() {
        let schema = any().options(&[1i32.to_value(), 2i32.to_value()]);
        let result = schema.validate(&3i32.to_value().into());
        assert!(result.is_err());
    }

    #[test]
    fn validate_required_and_options() {
        let schema = any()
            .required()
            .options(&[true.to_value(), false.to_value()]);

        assert!(schema.validate(&true.to_value().into()).is_ok());
        assert!(schema.validate(&false.to_value().into()).is_ok());
        assert!(schema.validate(&xval::valueof!(null).into()).is_err());
        assert!(schema.validate(&42i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_collects_multiple_errors() {
        let schema = any().required().equals(true.to_value());
        let err = schema.validate(&xval::valueof!(null).into()).unwrap_err();
        assert_eq!(err.errors.len(), 2);
    }
}
