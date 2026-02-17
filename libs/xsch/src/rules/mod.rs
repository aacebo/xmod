mod one_of;
mod required;

pub use one_of::*;
pub use required::*;

use crate::{Context, ValidError, Validate};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum Rule {
    OneOf(OneOf),
    Required(Required),
}

impl Rule {
    pub fn name(&self) -> &str {
        match self {
            Self::OneOf(_) => "one_of",
            Self::Required(_) => "required",
        }
    }
}

impl Validate for Rule {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::OneOf(v) => v.validate(ctx),
            Self::Required(v) => v.validate(ctx),
        }
    }
}
