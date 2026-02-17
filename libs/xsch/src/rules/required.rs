use crate::{AnySchema, Context, ValidError, Validate, rules::Rule};

#[repr(transparent)]
#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Required(bool);

impl Required {
    pub fn new() -> Self {
        Self(true)
    }
}

impl From<Required> for Rule {
    fn from(value: Required) -> Self {
        Self::Required(value)
    }
}

impl Validate for Required {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if ctx.value.is_null() {
            return Err(ctx.error("required"));
        }

        Ok(ctx.value.clone())
    }
}

impl AnySchema {
    pub fn required(self) -> Self {
        self.rule(Required::new().into())
    }
}
