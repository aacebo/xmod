use xval::AsValue;

use crate::{
    Context, Equals, NumberSchema, Options, Required, RuleSet, Schema, ValidError, Validate,
};

pub fn float() -> FloatSchema {
    FloatSchema::default()
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct FloatSchema(pub(crate) RuleSet);

impl FloatSchema {
    pub fn equals(mut self, value: xval::Float) -> Self {
        self.0 = self.0.add(Equals::from(value.as_value()).into());
        self
    }

    pub fn options(mut self, options: &[xval::Float]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.as_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }
}

impl From<FloatSchema> for Schema {
    fn from(value: FloatSchema) -> Self {
        Self::Float(value)
    }
}

impl From<NumberSchema> for FloatSchema {
    fn from(value: NumberSchema) -> Self {
        Self(value.0)
    }
}

impl Validate for FloatSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let value = self.0.validate(ctx)?;

        if !value.is_null() && !value.is_float() {
            return Err(ctx.error("expected float"));
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_float() {
        let schema = float();
        assert!(schema.validate(&3.14f64.as_value().into()).is_ok());
    }

    #[test]
    fn validate_rejects_int() {
        let schema = float();
        let err = schema.validate(&42i32.as_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected float"));
    }

    #[test]
    fn validate_rejects_string() {
        let schema = float();
        let err = schema.validate(&"hello".as_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected float"));
    }

    #[test]
    fn validate_null_passes_without_required() {
        let schema = float();
        assert!(schema.validate(&xval::Value::Null.into()).is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = float().required();
        let err = schema.validate(&xval::Value::Null.into()).unwrap_err();
        assert_eq!(err.errors[0].message.as_deref(), Some("required"));
    }

    #[test]
    fn validate_equals() {
        let schema = float().equals(xval::Float::from_f64(3.14));
        assert!(schema.validate(&3.14f64.as_value().into()).is_ok());
        assert!(schema.validate(&2.71f64.as_value().into()).is_err());
    }

    #[test]
    fn validate_options() {
        let schema = float().options(&[
            xval::Float::from_f64(1.0),
            xval::Float::from_f64(2.5),
            xval::Float::from_f64(3.14),
        ]);
        assert!(schema.validate(&2.5f64.as_value().into()).is_ok());
        assert!(schema.validate(&4.0f64.as_value().into()).is_err());
    }

    #[test]
    fn validate_required_and_equals() {
        let schema = float().required().equals(xval::Float::from_f64(1.5));
        assert!(schema.validate(&1.5f64.as_value().into()).is_ok());
        assert!(schema.validate(&2.5f64.as_value().into()).is_err());
        assert!(schema.validate(&xval::Value::Null.into()).is_err());
    }

    #[test]
    fn from_number_schema() {
        let schema: FloatSchema = NumberSchema::default().into();
        assert!(schema.validate(&3.14f64.as_value().into()).is_ok());
    }
}
