mod action;
mod context;
mod error;
mod event;
mod invoke;

pub use action::*;
pub use context::*;
pub use error::*;
pub use event::*;
pub use invoke::*;

use async_trait::async_trait;

#[async_trait]
pub trait Execute: Send + Sync {
    async fn exec(&self, ctx: &mut Context) -> xok::Result<xval::Value>;
}

pub trait Subscribe: Send + Sync {
    fn subscribe(&mut self, action: impl Execute);
}
