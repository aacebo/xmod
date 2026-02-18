use xval::{AsValue, UInt};

use crate::{Context, Equals, Max, Min, Options, Required, RuleSet, Schema, ValidError, Validate};

pub fn string() -> StringSchema {
    StringSchema::default()
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct StringSchema(RuleSet);

impl StringSchema {
    pub fn equals(mut self, value: &str) -> Self {
        self.0 = self.0.add(Equals::from(value.as_value()).into());
        self
    }

    pub fn options(mut self, options: &[String]) -> Self {
        self.0 = self
            .0
            .add(Options::from(options.iter().map(|v| v.as_value()).collect::<Vec<_>>()).into());
        self
    }

    pub fn required(mut self) -> Self {
        self.0 = self.0.add(Required::new(true).into());
        self
    }

    pub fn min(mut self, min: usize) -> Self {
        self.0 = self.0.add(Min::from(UInt::from_usize(min)).into());
        self
    }

    pub fn max(mut self, max: usize) -> Self {
        self.0 = self.0.add(Max::from(UInt::from_usize(max)).into());
        self
    }
}

impl From<StringSchema> for Schema {
    fn from(value: StringSchema) -> Self {
        Self::String(value)
    }
}

impl Validate for StringSchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let value = self.0.validate(ctx)?;

        if !value.is_string() {
            return Err(ctx.error("expected string"));
        }

        Ok(value)
    }
}
