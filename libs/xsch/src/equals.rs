use crate::{Context, Rule, ValidError, Validate};

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Equals(xval::Value);

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

impl Validate for Equals {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if ctx.value != self.0 {
            return Err(ctx.error(&format!("{} is not equal to {}", &ctx.value, &self.0)));
        }

        Ok(ctx.value.clone())
    }
}
