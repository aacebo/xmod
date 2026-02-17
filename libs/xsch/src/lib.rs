mod bool;
mod context;
mod error;
mod options;
mod required;
mod rule;
mod equals;

pub use bool::*;
pub use context::*;
pub use error::*;
pub use options::*;
pub use required::*;
pub use rule::*;
pub use equals::*;

pub trait Validate {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError>;
}

#[derive(Debug, Clone)]
pub enum Schema {
    Any,
    Bool(BoolSchema),
}

impl Validate for Schema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::Any => Ok(ctx.value.clone()),
            Self::Bool(v) => v.validate(ctx),
        }
    }
}
