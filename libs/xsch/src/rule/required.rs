use crate::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Required(bool);

impl Required {
    pub const KEY: &str = "required";
    pub const PHASE: crate::Phase = crate::Phase::Presence;

    pub fn new(is_required: bool) -> Self {
        Self(is_required)
    }
}

impl From<Required> for Rule {
    fn from(value: Required) -> Self {
        Self::Required(value)
    }
}

impl Validator for Required {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if self.0 && ctx.value.is_null() {
            return Err(ctx.error("required"));
        }

        Ok(ctx.value.clone())
    }
}
