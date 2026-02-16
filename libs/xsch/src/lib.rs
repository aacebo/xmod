mod bool;
mod context;
mod error;
pub mod rules;

pub use bool::*;
pub use context::*;
pub use error::*;

pub trait Validate {
    fn validate(&self, input: &xval::Value) -> Result<xval::Value, String>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Any {}
