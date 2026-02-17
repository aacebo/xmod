use crate::{AnySchema, Context, ValidError, Validate, rules::Rule};

#[repr(transparent)]
#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct OneOf(Vec<xval::Value>);

impl OneOf {
    pub fn new(items: &[xval::Value]) -> Self {
        Self(items.to_vec())
    }
}

impl From<OneOf> for Rule {
    fn from(value: OneOf) -> Self {
        Self::OneOf(value)
    }
}

impl Validate for OneOf {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        for option in &self.0 {
            if ctx.value == *option {
                return Ok(ctx.value.clone());
            }
        }

        let options = self
            .0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(ctx.error(&format!("must be one of [{}]", options)))
    }
}

impl AnySchema {
    pub fn options(self, items: &[xval::Value]) -> Self {
        self.rule(OneOf::new(items).into())
    }
}
