use xtera::Scope;
use xtera_derive::render;

#[test]
fn if_true() {
    let tpl = render! { @if (true) { "yes" } };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "yes");
}

#[test]
fn if_false_renders_nothing() {
    let tpl = render! { @if (false) { "yes" } };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "");
}

#[test]
fn if_else() {
    let tpl = render! {
        @if (show) { "visible" } @else { "hidden" }
    };

    let mut s = Scope::new();
    s.set_var("show", xval::valueof!(true));
    assert_eq!(tpl.render(&s).unwrap(), "visible");

    s.set_var("show", xval::valueof!(false));
    assert_eq!(tpl.render(&s).unwrap(), "hidden");
}

#[test]
fn if_else_if_else() {
    let tpl = render! {
        @if (x == 1) { "one" }
        @else if (x == 2) { "two" }
        @else { "other" }
    };

    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(2_i64));
    assert_eq!(tpl.render(&s).unwrap(), "two");

    s.set_var("x", xval::valueof!(99_i64));
    assert_eq!(tpl.render(&s).unwrap(), "other");
}

#[test]
fn unary_not_in_condition() {
    let tpl = render! { @if (!hidden) { "shown" } };
    let mut s = Scope::new();
    s.set_var("hidden", xval::valueof!(false));
    assert_eq!(tpl.render(&s).unwrap(), "shown");
}

#[test]
fn comparison_operators_in_condition() {
    let tpl = render! { @if (x >= 10 && x <= 20) { "in range" } };
    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(15_i64));
    assert_eq!(tpl.render(&s).unwrap(), "in range");
}
