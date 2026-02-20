use xtera::Scope;
use xtera_derive::render;

#[test]
fn simple_interpolation() {
    let tpl = render! { "x=" {{ x }} };
    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(42_i64));
    assert_eq!(tpl.render(&s).unwrap(), "x=42");
}

#[test]
fn boolean_and_null_literals() {
    let tpl = render! { {{ true }} {{ false }} {{ null }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "truefalse<null>");
}

#[test]
fn whitespace_inside_braces() {
    let tpl = render! { {{   x   }} };
    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(1_i64));
    assert_eq!(tpl.render(&s).unwrap(), "1");
}

#[test]
fn whitespace_around_operators() {
    let tpl = render! { {{  1  +  2  }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "3");
}

#[test]
fn empty_template() {
    let tpl = render! {};
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "");
}

#[test]
fn multiple_text_nodes() {
    let tpl = render! { "hello" " " "world" };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "hello world");
}
