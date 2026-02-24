use std::{collections::BTreeMap, sync::Arc};

use xval::{ToValue, Value};

use crate::{ActionRef, Execute, FluxError};

#[derive(Clone)]
pub struct Context {
    pub started_at: std::time::Instant,
    pub input: Value,

    data: BTreeMap<String, Value>,
    actions: BTreeMap<ActionRef, Arc<dyn Execute>>,
}

impl Context {
    pub fn new(input: impl ToValue) -> Self {
        Self {
            started_at: std::time::Instant::now(),
            input: input.to_value(),
            data: BTreeMap::new(),
            actions: BTreeMap::new(),
        }
    }

    pub fn elapse(&self) -> std::time::Duration {
        std::time::Instant::now() - self.started_at
    }

    pub fn data(&self) -> &BTreeMap<String, Value> {
        &self.data
    }

    pub fn var(&mut self, name: &str, value: xval::Value) -> &mut Self {
        self.data.insert(name.to_string(), value);
        self
    }

    pub fn registry(&mut self, action: ActionRef, executor: impl Execute + 'static) -> &mut Self {
        self.actions.insert(action, Arc::new(executor));
        self
    }

    pub fn merge(&mut self, other: Context) -> &mut Self {
        for (key, value) in other.data {
            self.data.insert(key, value);
        }

        self
    }
}

impl Context {
    pub async fn execute(
        &mut self,
        action: &ActionRef,
        input: xval::Value,
    ) -> xok::Result<xval::Value> {
        let executor = match self.actions.get(action) {
            None => return Err(Box::new(FluxError::new("action not found"))),
            Some(v) => v.clone(),
        };

        let mut ctx = self.clone();
        ctx.input = input;
        Ok(executor.exec(&mut ctx).await?)
    }
}
