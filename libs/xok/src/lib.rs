mod code;
mod severity;

pub use code::*;
pub use severity::*;

use std::borrow::Cow;

pub type Result<T> = std::result::Result<T, Box<dyn XError>>;

pub trait XError: std::error::Error {
    fn name(&self) -> &'static str;
    fn module(&self) -> &'static str;

    fn code(&self) -> Code {
        Code::Internal
    }

    fn severity(&self) -> Severity {
        Severity::Medium
    }

    fn message(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
