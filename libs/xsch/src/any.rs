use crate::{Context, Equals, Options, Required, RuleSet, Schema, ValidError, Validate};

#[derive(Debug, Default, Clone)]
pub struct AnySchema(RuleSet);

impl AnySchema {
    pub fn equals(mut self, value: xval::Value) -> Self {
        self.0 = self.0.add(Equals::from(value).into());
        self
    }

    pub fn options(mut self, options: &[xval::Value]) -> Self {
        self.0 = self.0.add(Options::from(options).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }
}

impl From<AnySchema> for Schema {
    fn from(value: AnySchema) -> Self {
        Self::Any(value)
    }
}

impl Validate for AnySchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        self.0.validate(ctx)
    }
}
