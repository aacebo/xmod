use xtera::Scope;
use xtera_derive::render;

#[test]
fn basic_include() {
    let header = render! { "<h1>" {{ title }} "</h1>" };
    let page = render! { @include("header") "<p>body</p>" };

    let mut s = Scope::new();
    s.set_var("title", xval::valueof!("Hello"));
    s.set_template("header", header);
    s.set_template("page", page);
    assert_eq!(s.render("page").unwrap(), "<h1>Hello</h1><p>body</p>");
}
