use crate::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Equals(xval::Value);

impl Equals {
    pub const KEY: &str = "equals";
}

impl From<xval::Value> for Equals {
    fn from(value: xval::Value) -> Self {
        Self(value)
    }
}

impl From<Equals> for Rule {
    fn from(value: Equals) -> Self {
        Self::Equals(value)
    }
}

impl Validator for Equals {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if ctx.value != self.0 {
            return Err(ctx.error(&format!("{} is not equal to {}", &ctx.value, &self.0)));
        }

        Ok(ctx.value.clone())
    }
}
