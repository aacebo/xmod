use std::{collections::BTreeMap, sync::Arc};

use crate::{Template, eval};

pub trait Pipe {
    fn invoke(&self, this: &xval::Value, args: &[xval::Value]) -> eval::Result<xval::Value>;
}

pub trait Func {
    fn invoke(&self, args: &[xval::Value]) -> eval::Result<xval::Value>;
}

#[derive(Default, Clone)]
pub struct Scope {
    vars: BTreeMap<String, Arc<xval::Value>>,
    pipes: BTreeMap<String, Arc<dyn Pipe>>,
    funcs: BTreeMap<String, Arc<dyn Func>>,
    templates: BTreeMap<String, Arc<Template>>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn var(&self, name: &str) -> Option<&xval::Value> {
        self.vars.get(name).map(|v| &**v)
    }

    pub fn pipe(&self, name: &str) -> Option<&dyn Pipe> {
        self.pipes.get(name).map(|v| &**v)
    }

    pub fn func(&self, name: &str) -> Option<&dyn Func> {
        self.funcs.get(name).map(|v| &**v)
    }

    pub fn template(&self, name: &str) -> Option<&Template> {
        self.templates.get(name).map(|v| &**v)
    }

    pub fn set_var(&mut self, name: &str, value: xval::Value) -> &mut Self {
        self.vars.insert(name.to_string(), Arc::new(value));
        self
    }

    pub fn set_pipe<P: Pipe + 'static>(&mut self, name: &str, pipe: P) -> &mut Self {
        self.pipes.insert(name.to_string(), Arc::new(pipe));
        self
    }

    pub fn set_func<F: Func + 'static>(&mut self, name: &str, func: F) -> &mut Self {
        self.funcs.insert(name.to_string(), Arc::new(func));
        self
    }

    pub fn set_template(&mut self, name: &str, template: Template) -> &mut Self {
        self.templates.insert(name.to_string(), Arc::new(template));
        self
    }

    pub fn render(&self, name: &str) -> eval::Result<String> {
        self.template(name)
            .expect("template not found")
            .render(self)
    }
}
