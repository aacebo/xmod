mod severity;

pub use severity::*;

use std::borrow::Cow;

pub type Result<T> = std::result::Result<T, Box<dyn XError>>;

pub trait XError: std::error::Error {
    fn name(&self) -> &'static str;
    fn code(&self) -> u16;
    fn category(&self) -> &'static str;

    fn severity(&self) -> Severity {
        Severity::Medium
    }

    fn message(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}
