use crate::{AnySchema, Context, ValidError, Validate};

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

impl AnySchema {
    pub fn required(self) -> Self {
        self.rule("required", Required::new())
    }
}
