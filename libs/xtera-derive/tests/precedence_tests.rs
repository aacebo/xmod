use xtera::Scope;
use xtera_derive::render;

#[test]
fn mul_before_add() {
    let tpl = render! { {{ 2 + 3 * 4 }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "14");
}

#[test]
fn comparison_before_logical() {
    let tpl = render! { {{ 1 < 2 && 3 > 1 }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "true");
}

#[test]
fn or_lowest() {
    let tpl = render! { {{ false || true && false }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "false");
}

#[test]
fn parens_override_precedence() {
    let tpl = render! { {{ (2 + 3) * 4 }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "20");
}

#[test]
fn mod_same_as_mul() {
    let tpl = render! { {{ 10 % 3 + 1 }} };
    assert_eq!(tpl.render(&Scope::new()).unwrap(), "2");
}
