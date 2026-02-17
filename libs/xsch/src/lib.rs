mod any;
mod context;
mod error;
pub mod rules;

pub use any::*;
pub use context::*;
pub use error::*;

pub trait Validate {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError>;
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum Schema {
    Any(AnySchema),
}
