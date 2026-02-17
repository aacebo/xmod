use crate::{Context, Rule, ValidError, Validate};

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Required(bool);

impl Required {
    pub fn new(is_required: bool) -> Self {
        Self(is_required)
    }
}

impl From<Required> for Rule {
    fn from(value: Required) -> Self {
        Self::Required(value)
    }
}

impl Validate for Required {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if self.0 && ctx.value.is_null() {
            return Err(ctx.error("required"));
        }

        Ok(ctx.value.clone())
    }
}
