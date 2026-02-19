use xtera::{Func, Pipe, Scope, Template, derive::render};

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
fn simple_text() {
    let tpl = render! { "hello world" };
    let s = Scope::new();
    assert_eq!(tpl.render(&s).unwrap(), "hello world");
}

#[test]
fn interpolation() {
    let tpl = render! { "x=" {{ x }} };
    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(42_i64));
    assert_eq!(tpl.render(&s).unwrap(), "x=42");
}

#[test]
fn pipe_expression() {
    let tpl = render! { {{ name | upper }} };
    let mut s = scope();
    s.set_var("name", xval::valueof!("alice"));
    assert_eq!(tpl.render(&s).unwrap(), "ALICE");
}

#[test]
fn if_else() {
    let tpl = render! {
        @if (show) {
            "visible"
        } @else {
            "hidden"
        }
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
        @if (x == 1) {
            "one"
        } @else if (x == 2) {
            "two"
        } @else {
            "other"
        }
    };

    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(2_i64));
    assert_eq!(tpl.render(&s).unwrap(), "two");
}

#[test]
fn for_loop() {
    let tpl = render! {
        @for (item of items; track item) {
            "[" {{ item }} "]"
        }
    };

    let mut s = Scope::new();
    s.set_var("items", xval::valueof!([1_i64, 2_i64, 3_i64]));

    assert_eq!(tpl.render(&s).unwrap(), "[1][2][3]");
}

#[test]
fn match_statement() {
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

    s.set_var("color", xval::valueof!("green"));
    assert_eq!(tpl.render(&s).unwrap(), "?");
}

#[test]
fn include_template() {
    let header = render! { "<h1>" {{ title | upper }} "</h1>" };
    let page = render! {
        @include("header")
        "<p>content</p>"
    };

    let mut s = scope();
    s.set_var("title", xval::valueof!("Hello"));
    s.set_template("header", header);
    s.set_template("page", page);

    assert_eq!(s.render("page").unwrap(), "<h1>HELLO</h1><p>content</p>");
}

#[test]
fn binary_expression() {
    let tpl = render! { {{ x * 2 + 1 }} };
    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(5_i64));
    assert_eq!(tpl.render(&s).unwrap(), "11");
}

#[test]
fn nested_for_with_if() {
    let tpl = render! {
        @for (n of items; track n) {
            @if (n % 2 == 0) {
                "even"
            } @else {
                "odd"
            }
        }
    };

    let mut s = Scope::new();
    s.set_var("items", xval::valueof!([1_i64, 2_i64, 3_i64, 4_i64]));

    assert_eq!(tpl.render(&s).unwrap(), "oddevenoddeven");
}

#[test]
fn boolean_and_null_literals() {
    let tpl = render! { {{ true }} {{ false }} {{ null }} };
    let s = Scope::new();
    assert_eq!(tpl.render(&s).unwrap(), "truefalse<null>");
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
fn unary_not() {
    let tpl = render! {
        @if (!hidden) { "shown" }
    };
    let mut s = Scope::new();
    s.set_var("hidden", xval::valueof!(false));
    assert_eq!(tpl.render(&s).unwrap(), "shown");
}

#[test]
fn comparison_operators() {
    let tpl = render! {
        @if (x >= 10 && x <= 20) { "in range" }
    };
    let mut s = Scope::new();
    s.set_var("x", xval::valueof!(15_i64));
    assert_eq!(tpl.render(&s).unwrap(), "in range");
}

#[test]
fn matches_runtime_parser() {
    // Verify macro output matches Template::parse for the same logic
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
