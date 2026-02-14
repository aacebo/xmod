use async_trait::async_trait;
use xok::Result;
use xval::Value;

use crate::Context;

#[async_trait]
pub trait Action: Send + Sync {
    async fn exec(&self, ctx: &mut Context) -> Result<Value>;
}
