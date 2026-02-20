mod control_tests;
mod expr_tests;
mod include_tests;
mod interp_tests;
mod loop_tests;
mod match_tests;
mod precedence_tests;

use xtera::{Func, Pipe, Scope, Template};
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
fn nested_for_with_if_and_pipes() {
    let tpl = render! {
        @for (user of users; track user) {
            @if (user.active) {
                {{ user.name | upper }}
            } @else {
                "inactive"
            }
        }
    };

    let mut s = scope();
    s.set_var(
        "users",
        xval::valueof!([
            { "name": "alice", "active": true },
            { "name": "bob", "active": false }
        ]),
    );
    s.set_template("main", tpl);
    assert_eq!(s.render("main").unwrap(), "ALICEinactive");
}

#[test]
fn full_page_with_include_match_for() {
    let header = render! { "<h1>" {{ title | upper }} "</h1>" };
    let page = render! {
        @include("header")
        @match (theme) {
            "dark" => { "<body class='dark'>" },
            _ => { "<body>" }
        }
        @for (item of items; track item) {
            "<p>" {{ item }} "</p>"
        }
        "</body>"
    };

    let mut s = scope();
    s.set_var("title", xval::valueof!("My Page"));
    s.set_var("theme", xval::valueof!("dark"));
    s.set_var("items", xval::valueof!(["a", "b"]));
    s.set_template("header", header);
    s.set_template("page", page);

    assert_eq!(
        s.render("page").unwrap(),
        "<h1>MY PAGE</h1><body class='dark'><p>a</p><p>b</p></body>"
    );
}

#[test]
fn matches_runtime_parser() {
    let macro_tpl = render! {
        @for (n of items; track n) {
            @if (n % 2 == 0) { "even" } @else { "odd" }
        }
    };
    let runtime_tpl =
        Template::parse("@for (n of items; track n) {@if (n % 2 == 0) {even}@else{odd}}").unwrap();

    let mut s = Scope::new();
    s.set_var("items", xval::valueof!([1_i64, 2_i64, 3_i64]));
    s.set_template("macro", macro_tpl);
    s.set_template("runtime", runtime_tpl);
    assert_eq!(s.render("macro").unwrap(), s.render("runtime").unwrap());
}
