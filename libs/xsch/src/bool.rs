use xval::ToValue;

use crate::{AsSchema, Context, Equals, Options, Required, RuleSet, Schema, ValidError, Validator};

pub fn bool() -> BoolSchema {
    BoolSchema::default()
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct BoolSchema(pub(crate) RuleSet);

impl BoolSchema {
    pub fn equals(mut self, value: bool) -> Self {
        self.0 = self.0.add(Equals::from(value.to_value()).into());
        self
    }

    pub fn options(mut self, options: &[bool]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.to_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }
}

impl AsSchema for BoolSchema {
    fn as_schema(&self) -> Schema {
        Schema::Bool(self.clone())
    }
}

impl From<BoolSchema> for Schema {
    fn from(value: BoolSchema) -> Self {
        Self::Bool(value)
    }
}

impl Validator for BoolSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let value = self.0.validate(ctx)?;

        if !value.is_null() && !value.is_bool() {
            return Err(ctx.error("expected bool"));
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_bool() {
        let schema = bool();
        assert!(schema.validate(&true.to_value().into()).is_ok());
        assert!(schema.validate(&false.to_value().into()).is_ok());
    }

    #[test]
    fn validate_rejects_non_bool() {
        let schema = bool();
        let err = schema.validate(&42i32.to_value().into()).unwrap_err();
        assert_eq!(err.message.as_deref(), Some("expected bool"));
    }

    #[test]
    fn validate_rejects_string() {
        let schema = bool();
        assert!(schema.validate(&"true".to_value().into()).is_err());
    }

    #[test]
    fn validate_null_passes_without_required() {
        let schema = bool();
        assert!(schema.validate(&xval::valueof!(null).into()).is_ok());
    }

    #[test]
    fn validate_required_rejects_null() {
        let schema = bool().required();
        let err = schema.validate(&xval::valueof!(null).into()).unwrap_err();
        assert_eq!(err.errors[0].message.as_deref(), Some("required"));
    }

    #[test]
    fn validate_equals() {
        let schema = bool().equals(true);
        assert!(schema.validate(&true.to_value().into()).is_ok());
        assert!(schema.validate(&false.to_value().into()).is_err());
    }

    #[test]
    fn validate_options() {
        let schema = bool().options(&[true]);
        assert!(schema.validate(&true.to_value().into()).is_ok());
        assert!(schema.validate(&false.to_value().into()).is_err());
    }

    #[test]
    fn validate_required_and_equals() {
        let schema = bool().required().equals(false);
        assert!(schema.validate(&false.to_value().into()).is_ok());
        assert!(schema.validate(&true.to_value().into()).is_err());
        assert!(schema.validate(&xval::valueof!(null).into()).is_err());
    }
}
