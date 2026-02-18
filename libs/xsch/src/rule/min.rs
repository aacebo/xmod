use crate::{Context, Rule, ValidError, Validate};

#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Min(xval::UInt);

impl Min {
    pub const KEY: &str = "min";
}

impl From<xval::UInt> for Min {
    fn from(value: xval::UInt) -> Self {
        Self(value)
    }
}

impl From<Min> for Rule {
    fn from(value: Min) -> Self {
        Self::Min(value)
    }
}

impl Validate for Min {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if ctx.value.len() < self.0.to_u64() as usize {
            return Err(ctx.error(&format!(
                "expected min of {}, received {}",
                &self.0, &ctx.value
            )));
        }

        Ok(ctx.value.clone())
    }
}
