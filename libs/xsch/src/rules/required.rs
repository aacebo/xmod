use crate::{Context, ValidError, Validate, rules::RuleRegistry};

#[derive(Debug, Default, Clone)]
pub struct Required;

impl Required {
    pub fn new() -> Self {
        Self
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

impl RuleRegistry {
    pub fn required(&mut self) -> &mut Self {
        self.register("required", Required::new())
    }
}
