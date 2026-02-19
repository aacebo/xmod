use crate::{Context, Rule, ValidError, Validator};

#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Max(xval::Number);

impl Max {
    pub const KEY: &str = "max";

    pub fn new(value: xval::Number) -> Self {
        Self(value)
    }
}

impl From<xval::Number> for Max {
    fn from(value: xval::Number) -> Self {
        Self(value)
    }
}

impl From<Max> for Rule {
    fn from(value: Max) -> Self {
        Self::Max(value)
    }
}

impl Validator for Max {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if ctx.value.is_array() || ctx.value.is_string() {
            if ctx.value.len() > self.0.to_usize() {
                return Err(ctx.error(&format!(
                    "expected max of {}, received {}",
                    &self.0, &ctx.value
                )));
            }
        }

        if ctx.value.is_number() {
            if ctx.value.as_number() > &self.0 {
                return Err(ctx.error(&format!(
                    "expected max of {}, received {}",
                    &self.0, &ctx.value
                )));
            }
        }

        Ok(ctx.value.clone())
    }
}
