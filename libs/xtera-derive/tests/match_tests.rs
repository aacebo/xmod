use xtera::Scope;
use xtera_derive::render;

#[test]
fn match_arm() {
    let tpl = render! {
        @match (color) {
            "red" => { "R" },
            "blue" => { "B" },
            _ => { "?" }
        }
    };

    let mut s = Scope::new();
    s.set_var("color", xval::valueof!("blue"));
    assert_eq!(tpl.render(&s).unwrap(), "B");
}

#[test]
fn match_default() {
    let tpl = render! {
        @match (color) {
            "red" => { "R" },
            _ => { "?" }
        }
    };

    let mut s = Scope::new();
    s.set_var("color", xval::valueof!("green"));
    assert_eq!(tpl.render(&s).unwrap(), "?");
}

#[test]
fn match_no_match_no_default() {
    let tpl = render! {
        @match (x) {
            "a" => { "A" }
        }
    };

    let mut s = Scope::new();
    s.set_var("x", xval::valueof!("z"));
    s.set_template("main", tpl);
    assert_eq!(s.render("main").unwrap(), "");
}
