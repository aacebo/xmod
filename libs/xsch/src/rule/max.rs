use crate::{Context, Rule, ValidError, Validate};

#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Max(xval::UInt);

impl Max {
    pub const KEY: &str = "max";
}

impl From<xval::UInt> for Max {
    fn from(value: xval::UInt) -> Self {
        Self(value)
    }
}

impl From<Max> for Rule {
    fn from(value: Max) -> Self {
        Self::Max(value)
    }
}

impl Validate for Max {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if ctx.value.len() > self.0.to_u64() as usize {
            return Err(ctx.error(&format!(
                "expected max of {}, received {}",
                &self.0, &ctx.value
            )));
        }

        Ok(ctx.value.clone())
    }
}
