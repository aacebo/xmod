use xval::AsValue;

use crate::{Context, Equals, RuleSet, ValidError, Validate};

#[derive(Debug, Default, Clone)]
pub struct BoolSchema(RuleSet);

impl BoolSchema {
    pub fn equals(mut self, value: bool) -> Self {
        self.0 = self.0.add(Equals::from(value.as_value()).into());
        self
    }
}

impl Validate for BoolSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        if !ctx.value.is_bool() {
            return Err(ctx.error("expected bool"));
        }

        self.0.validate(ctx)
    }
}
