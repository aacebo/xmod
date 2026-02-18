use std::{collections::BTreeMap, sync::Arc};

use crate::{Template, ast};

pub trait Pipe {
    fn invoke(&self, this: &xval::Value, args: &[xval::Value]) -> ast::Result<xval::Value>;
}

pub trait Func {
    fn invoke(&self, args: &[xval::Value]) -> ast::Result<xval::Value>;
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

    pub fn render(&self, name: &str) -> ast::Result<String> {
        self.template(name)
            .expect("template not found")
            .render(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NoopPipe;
    impl Pipe for NoopPipe {
        fn invoke(&self, val: &xval::Value, _args: &[xval::Value]) -> ast::Result<xval::Value> {
            Ok(val.clone())
        }
    }

    struct ConstFunc(xval::Value);
    impl Func for ConstFunc {
        fn invoke(&self, _args: &[xval::Value]) -> ast::Result<xval::Value> {
            Ok(self.0.clone())
        }
    }

    #[test]
    fn var_set_and_get() {
        let mut scope = Scope::new();
        scope.set_var("x", xval::valueof!(42_i64));
        assert_eq!(*scope.var("x").unwrap(), 42i64);
    }

    #[test]
    fn var_missing_returns_none() {
        let scope = Scope::new();
        assert!(scope.var("x").is_none());
    }

    #[test]
    fn var_overwrite() {
        let mut scope = Scope::new();
        scope.set_var("x", xval::valueof!(1_i64));
        scope.set_var("x", xval::valueof!(2_i64));
        assert_eq!(*scope.var("x").unwrap(), 2i64);
    }

    #[test]
    fn pipe_set_and_get() {
        let mut scope = Scope::new();
        scope.set_pipe("noop", NoopPipe);
        assert!(scope.pipe("noop").is_some());
    }

    #[test]
    fn pipe_missing_returns_none() {
        let scope = Scope::new();
        assert!(scope.pipe("noop").is_none());
    }

    #[test]
    fn func_set_and_get() {
        let mut scope = Scope::new();
        scope.set_func("greet", ConstFunc(xval::valueof!("hi")));
        assert!(scope.func("greet").is_some());
    }

    #[test]
    fn func_missing_returns_none() {
        let scope = Scope::new();
        assert!(scope.func("greet").is_none());
    }

    #[test]
    fn template_set_and_render() {
        let mut scope = Scope::new();
        scope.set_var("name", xval::valueof!("world"));
        let tpl = Template::parse("hello {{ name }}").unwrap();
        scope.set_template("greeting", tpl);
        assert_eq!(scope.render("greeting").unwrap(), "hello world");
    }

    #[test]
    fn include_template() {
        let mut scope = Scope::new();
        scope.set_var("name", xval::valueof!("world"));
        scope.set_template("header", Template::parse("Hello {{ name }}!").unwrap());
        scope.set_template(
            "page",
            Template::parse("@include('header') Welcome.").unwrap(),
        );
        assert_eq!(scope.render("page").unwrap(), "Hello world! Welcome.");
    }

    #[test]
    fn include_missing_template() {
        let mut scope = Scope::new();
        scope.set_template("page", Template::parse("@include('missing')").unwrap());
        assert!(scope.render("page").is_err());
    }

    #[test]
    fn clone_isolates_mutations() {
        let mut scope = Scope::new();
        scope.set_var("x", xval::valueof!(1_i64));
        let mut cloned = scope.clone();
        cloned.set_var("x", xval::valueof!(2_i64));
        cloned.set_var("y", xval::valueof!(3_i64));
        assert_eq!(*scope.var("x").unwrap(), 1i64);
        assert!(scope.var("y").is_none());
    }
}
