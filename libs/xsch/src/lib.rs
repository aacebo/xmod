mod any;
mod bool;
mod context;
mod equals;
mod error;
mod options;
mod required;
mod rule;

pub use any::*;
pub use bool::*;
pub use context::*;
pub use equals::*;
pub use error::*;
pub use options::*;
pub use required::*;
pub use rule::*;

pub trait Validate {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError>;
}

#[derive(Debug, Clone)]
pub enum Schema {
    Any(AnySchema),
    Bool(BoolSchema),
}

impl Validate for Schema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::Any(v) => v.validate(ctx),
            Self::Bool(v) => v.validate(ctx),
        }
    }
}
