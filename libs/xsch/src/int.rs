use xval::ToValue;

use crate::{
    Context, Equals, Max, Min, NumberSchema, Options, Required, RuleSet, Schema, ToSchema,
    ValidError, Validator,
};

pub fn int() -> IntSchema {
    IntSchema::default()
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct IntSchema(pub(crate) RuleSet);

impl IntSchema {
    pub fn equals(mut self, value: xval::Int) -> Self {
        self.0 = self.0.add(Equals::from(value.to_value()).into());
        self
    }

    pub fn options(mut self, options: &[xval::Int]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.to_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }

    pub fn min(mut self, min: isize) -> Self {
        self.0 = self.0.add(Min::from(xval::Number::from_isize(min)).into());
        self
    }

    pub fn max(mut self, max: isize) -> Self {
        self.0 = self.0.add(Max::from(xval::Number::from_isize(max)).into());
        self
    }
}

impl ToSchema for IntSchema {
    fn to_schema(&self) -> Schema {
        Schema::Int(self.clone())
    }
}

impl From<IntSchema> for Schema {
    fn from(value: IntSchema) -> Self {
        Self::Int(value)
    }
}

impl From<NumberSchema> for IntSchema {
    fn from(value: NumberSchema) -> Self {
        Self(value.0)
    }
}

impl Validator for IntSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let value = self.0.validate(ctx)?;

        if !value.is_null() && !value.is_int() {
            return Err(ctx.error("expected integer"));
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_int() {
        let schema = int();
        assert!(schema.validate(&42i32.to_value().into()).is_ok());
    }

    #[test]
    fn validate_rejects_float() {
        let schema = int();
        let err = schema.validate(&3.14f64.to_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected integer"));
    }

    #[test]
    fn validate_rejects_string() {
        let schema = int();
        let err = schema.validate(&"hello".to_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected integer"));
    }

    #[test]
    fn validate_null_passes_without_required() {
        let schema = int();
        assert!(schema.validate(&xval::valueof!(null).into()).is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = int().required();
        let err = schema.validate(&xval::valueof!(null).into()).unwrap_err();
        assert_eq!(err.errors[0].message.as_deref(), Some("required"));
    }

    #[test]
    fn validate_equals() {
        let schema = int().equals(xval::Int::from_i32(42));
        assert!(schema.validate(&42i32.to_value().into()).is_ok());
        assert!(schema.validate(&43i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_options() {
        let schema = int().options(&[
            xval::Int::from_i32(1),
            xval::Int::from_i32(2),
            xval::Int::from_i32(3),
        ]);
        assert!(schema.validate(&2i32.to_value().into()).is_ok());
        assert!(schema.validate(&4i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_required_and_equals() {
        let schema = int().required().equals(xval::Int::from_i32(10));
        assert!(schema.validate(&10i32.to_value().into()).is_ok());
        assert!(schema.validate(&11i32.to_value().into()).is_err());
        assert!(schema.validate(&xval::valueof!(null).into()).is_err());
    }

    #[test]
    fn validate_min() {
        let schema = int().min(5);
        assert!(schema.validate(&3i32.to_value().into()).is_err());
        assert!(schema.validate(&5i32.to_value().into()).is_ok());
        assert!(schema.validate(&10i32.to_value().into()).is_ok());
    }

    #[test]
    fn validate_max() {
        let schema = int().max(10);
        assert!(schema.validate(&5i32.to_value().into()).is_ok());
        assert!(schema.validate(&10i32.to_value().into()).is_ok());
        assert!(schema.validate(&15i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_min_and_max() {
        let schema = int().min(1).max(10);
        assert!(schema.validate(&0i32.to_value().into()).is_err());
        assert!(schema.validate(&1i32.to_value().into()).is_ok());
        assert!(schema.validate(&5i32.to_value().into()).is_ok());
        assert!(schema.validate(&10i32.to_value().into()).is_ok());
        assert!(schema.validate(&11i32.to_value().into()).is_err());
    }

    #[test]
    fn validate_min_negative() {
        let schema = int().min(-5);
        assert!(schema.validate(&(-10i32).to_value().into()).is_err());
        assert!(schema.validate(&(-5i32).to_value().into()).is_ok());
        assert!(schema.validate(&0i32.to_value().into()).is_ok());
    }

    #[test]
    fn from_number_schema() {
        let schema: IntSchema = NumberSchema::default().into();
        assert!(schema.validate(&42i32.to_value().into()).is_ok());
    }
}
