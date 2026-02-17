use xval::AsValue;

use crate::{Context, Equals, Options, Required, RuleSet, Schema, ValidError, Validate};

#[derive(Debug, Default, Clone)]
pub struct BoolSchema(RuleSet);

impl BoolSchema {
    pub fn equals(mut self, value: bool) -> Self {
        self.0 = self.0.add(Equals::from(value.as_value()).into());
        self
    }

    pub fn options(mut self, options: &[bool]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.as_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }
}

impl From<BoolSchema> for Schema {
    fn from(value: BoolSchema) -> Self {
        Self::Bool(value)
    }
}

impl Validate for BoolSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let value = self.0.validate(ctx)?;

        if !value.is_bool() {
            return Err(ctx.error("expected bool"));
        }

        Ok(value)
    }
}
