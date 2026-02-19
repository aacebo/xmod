use xval::AsValue;

use crate::{
    AsSchema, Context, Equals, FloatSchema, IntSchema, Max, Min, Options, Required, RuleSet,
    Schema, ValidError, Validate,
};

pub fn number() -> NumberSchema {
    NumberSchema::default()
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct NumberSchema(pub(crate) RuleSet);

impl NumberSchema {
    pub fn equals(mut self, value: xval::Number) -> Self {
        self.0 = self.0.add(Equals::from(value.as_value()).into());
        self
    }

    pub fn options(mut self, options: &[xval::Number]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.as_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }

    pub fn min(mut self, min: xval::Number) -> Self {
        self.0 = self.0.add(Min::from(min).into());
        self
    }

    pub fn max(mut self, max: xval::Number) -> Self {
        self.0 = self.0.add(Max::from(max).into());
        self
    }

    pub fn int(self) -> IntSchema {
        self.into()
    }

    pub fn float(self) -> FloatSchema {
        self.into()
    }
}

impl AsSchema for NumberSchema {
    fn as_schema(&self) -> Schema {
        Schema::Number(self.clone())
    }
}

impl From<NumberSchema> for Schema {
    fn from(value: NumberSchema) -> Self {
        Self::Number(value)
    }
}

impl Validate for NumberSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let value = self.0.validate(ctx)?;

        if !value.is_null() && !value.is_number() {
            return Err(ctx.error("expected number"));
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_int() {
        let schema = number();
        assert!(schema.validate(&42i32.as_value().into()).is_ok());
    }

    #[test]
    fn validate_float() {
        let schema = number();
        assert!(schema.validate(&3.14f64.as_value().into()).is_ok());
    }

    #[test]
    fn validate_rejects_non_number() {
        let schema = number();
        let err = schema.validate(&"hello".as_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected number"));
    }

    #[test]
    fn validate_null_passes_without_required() {
        let schema = number();
        assert!(schema.validate(&xval::valueof!(null).into()).is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = number().required();
        let err = schema.validate(&xval::valueof!(null).into()).unwrap_err();
        assert_eq!(err.errors[0].message.as_deref(), Some("required"));
    }

    #[test]
    fn validate_equals() {
        let schema = number().equals(xval::Number::from_i32(42));
        assert!(schema.validate(&42i32.as_value().into()).is_ok());
        assert!(schema.validate(&43i32.as_value().into()).is_err());
    }

    #[test]
    fn validate_options() {
        let schema = number().options(&[
            xval::Number::from_i32(1),
            xval::Number::from_i32(2),
            xval::Number::from_i32(3),
        ]);
        assert!(schema.validate(&2i32.as_value().into()).is_ok());
        assert!(schema.validate(&4i32.as_value().into()).is_err());
    }

    #[test]
    fn validate_required_and_equals() {
        let schema = number().required().equals(xval::Number::from_f64(3.14));
        assert!(schema.validate(&3.14f64.as_value().into()).is_ok());
        assert!(schema.validate(&2.0f64.as_value().into()).is_err());
        assert!(schema.validate(&xval::valueof!(null).into()).is_err());
    }

    #[test]
    fn validate_min() {
        let schema = number().min(xval::Number::from_i32(5));
        assert!(schema.validate(&3i32.as_value().into()).is_err());
        assert!(schema.validate(&5i32.as_value().into()).is_ok());
        assert!(schema.validate(&10i32.as_value().into()).is_ok());
    }

    #[test]
    fn validate_max() {
        let schema = number().max(xval::Number::from_i32(10));
        assert!(schema.validate(&5i32.as_value().into()).is_ok());
        assert!(schema.validate(&10i32.as_value().into()).is_ok());
        assert!(schema.validate(&15i32.as_value().into()).is_err());
    }

    #[test]
    fn validate_min_and_max() {
        let schema = number()
            .min(xval::Number::from_i32(1))
            .max(xval::Number::from_i32(10));
        assert!(schema.validate(&0i32.as_value().into()).is_err());
        assert!(schema.validate(&1i32.as_value().into()).is_ok());
        assert!(schema.validate(&5i32.as_value().into()).is_ok());
        assert!(schema.validate(&10i32.as_value().into()).is_ok());
        assert!(schema.validate(&11i32.as_value().into()).is_err());
    }

    #[test]
    fn validate_min_float() {
        let schema = number().min(xval::Number::from_f64(1.5));
        assert!(schema.validate(&1.0f64.as_value().into()).is_err());
        assert!(schema.validate(&1.5f64.as_value().into()).is_ok());
        assert!(schema.validate(&2.0f64.as_value().into()).is_ok());
    }

    #[test]
    fn validate_max_float() {
        let schema = number().max(xval::Number::from_f64(9.9));
        assert!(schema.validate(&9.0f64.as_value().into()).is_ok());
        assert!(schema.validate(&9.9f64.as_value().into()).is_ok());
        assert!(schema.validate(&10.0f64.as_value().into()).is_err());
    }

    #[test]
    fn convert_to_int_schema() {
        let schema = number().required().int();
        assert!(schema.validate(&42i32.as_value().into()).is_ok());
        assert!(schema.validate(&3.14f64.as_value().into()).is_err());
        assert!(schema.validate(&xval::valueof!(null).into()).is_err());
    }

    #[test]
    fn convert_to_float_schema() {
        let schema = number().required().float();
        assert!(schema.validate(&3.14f64.as_value().into()).is_ok());
        assert!(schema.validate(&42i32.as_value().into()).is_err());
        assert!(schema.validate(&xval::valueof!(null).into()).is_err());
    }
}
