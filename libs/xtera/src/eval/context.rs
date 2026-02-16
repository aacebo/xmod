use std::collections::BTreeMap;

use super::error::Result;

pub struct Context {
    vars: BTreeMap<String, xval::Value>,
    pipes: BTreeMap<String, Box<dyn Fn(&xval::Value, &[xval::Value]) -> Result<xval::Value>>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            vars: BTreeMap::new(),
            pipes: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, name: impl Into<String>, value: xval::Value) -> &mut Self {
        self.vars.insert(name.into(), value);
        self
    }

    pub fn set_pipe(
        &mut self,
        name: impl Into<String>,
        f: impl Fn(&xval::Value, &[xval::Value]) -> Result<xval::Value> + 'static,
    ) -> &mut Self {
        self.pipes.insert(name.into(), Box::new(f));
        self
    }

    pub(crate) fn get(&self, name: &str) -> Option<&xval::Value> {
        self.vars.get(name)
    }

    pub(crate) fn get_pipe(
        &self,
        name: &str,
    ) -> Option<&dyn Fn(&xval::Value, &[xval::Value]) -> Result<xval::Value>> {
        self.pipes.get(name).map(|b| b.as_ref())
    }

    pub(crate) fn child_scope(&self) -> BTreeMap<String, xval::Value> {
        self.vars.clone()
    }

    pub(crate) fn with_vars(
        &mut self,
        vars: BTreeMap<String, xval::Value>,
    ) -> BTreeMap<String, xval::Value> {
        std::mem::replace(&mut self.vars, vars)
    }
}
