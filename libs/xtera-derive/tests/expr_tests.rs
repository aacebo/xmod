use xtera::{Func, Pipe, Scope};
use xtera_derive::render;

struct UpperPipe;
impl Pipe for UpperPipe {
    fn invoke(&self, val: &xval::Value, _args: &[xval::Value]) -> xtera::ast::Result<xval::Value> {
        Ok(xval::valueof!((val.as_string().as_str().to_uppercase())))
    }
}

struct LenFunc;
impl Func for LenFunc {
    fn invoke(&self, args: &[xval::Value]) -> xtera::ast::Result<xval::Value> {
        Ok(xval::valueof!((args[0].as_array().len() as i64)))
    }
}

fn scope() -> Scope {
    let mut s = Scope::new();
    s.set_pipe("upper", UpperPipe);
    s.set_func("len", LenFunc);
    s
}

#[test]
fn binary_arithmetic() {
    let tpl = render! { {{ x * 2 + 1 }} };
    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(5_i64));
    assert_eq!(tpl.render(&s).unwrap(), "11");
}

#[test]
fn unary_negation() {
    let tpl = render! { {{ -5 }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "-5");
}

#[test]
fn string_concatenation() {
    let tpl = render! { {{ "a" + "b" }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "ab");
}

#[test]
fn parenthesized_grouping() {
    let tpl = render! { {{ (1 + 2) * 3 }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "9");
}

#[test]
fn float_literal() {
    let tpl = render! { {{ 3.14 }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "3.14");
}

#[test]
fn member_access() {
    let tpl = render! { {{ user.name }} };
    let mut s = Scope::new();
    s.set_var("user", xval::valueof!({ "name": "alice" }));
    assert_eq!(tpl.render(&s).unwrap(), "alice");
}

#[test]
fn array_index() {
    let tpl = render! { {{ items[1] }} };
    let mut s = Scope::new();
    s.set_var("items", xval::valueof!(["a", "b", "c"]));
    assert_eq!(tpl.render(&s).unwrap(), "b");
}

#[test]
fn function_call() {
    let tpl = render! { {{ len(items) }} };
    let mut s = scope();
    s.set_var("items", xval::valueof!([1_i64, 2_i64]));
    assert_eq!(tpl.render(&s).unwrap(), "2");
}

#[test]
fn pipe_expression() {
    let tpl = render! { {{ name | upper }} };
    let mut s = scope();
    s.set_var("name", xval::valueof!("alice"));
    assert_eq!(tpl.render(&s).unwrap(), "ALICE");
}
