pub mod ast;
pub mod parse;
mod scope;
mod template;

pub use scope::*;
pub use template::*;

#[cfg(feature = "derive")]
pub mod derive {
    pub use xtera_derive::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct UpperPipe;
    impl Pipe for UpperPipe {
        fn invoke(&self, val: &xval::Value, _args: &[xval::Value]) -> ast::Result<xval::Value> {
            Ok(xval::valueof!((val.as_string().as_str().to_uppercase())))
        }
    }

    struct LenFunc;
    impl Func for LenFunc {
        fn invoke(&self, args: &[xval::Value]) -> ast::Result<xval::Value> {
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
    fn nested_for_with_if() {
        let mut s = scope();
        s.set_var(
            "items",
            xval::valueof!(
                (vec![
                    xval::valueof!(1_i64),
                    xval::valueof!(2_i64),
                    xval::valueof!(3_i64),
                    xval::valueof!(4_i64),
                ])
            ),
        );

        let tpl = Template::parse("@for (n of items; track n) {@if (n % 2 == 0) {even}@else{odd}}")
            .unwrap();
        s.set_template("main", tpl);

        assert_eq!(s.render("main").unwrap(), "oddevenoddeven");
    }

    #[test]
    fn match_inside_for() {
        let mut s = scope();
        s.set_var(
            "colors",
            xval::valueof!(
                (vec![
                    xval::valueof!("red"),
                    xval::valueof!("blue"),
                    xval::valueof!("green"),
                ])
            ),
        );

        let tpl = Template::parse(
            "@for (c of colors; track c) {@match (c) { 'red' => {R}, 'blue' => {B}, _ => {?} }}",
        )
        .unwrap();
        s.set_template("main", tpl);

        assert_eq!(s.render("main").unwrap(), "RB?");
    }

    #[test]
    fn include_with_control_flow() {
        let mut s = scope();
        s.set_var("show", xval::valueof!(true));
        s.set_var("name", xval::valueof!("world"));

        s.set_template(
            "greeting",
            Template::parse("@if (show) {Hello {{ name | upper }}!}").unwrap(),
        );
        s.set_template(
            "page",
            Template::parse("[before]@include('greeting')[after]").unwrap(),
        );

        assert_eq!(s.render("page").unwrap(), "[before]Hello WORLD![after]");
    }

    #[test]
    fn pipes_and_expressions_in_control_flow() {
        let mut s = scope();
        s.set_var("x", xval::valueof!(10_i64));
        s.set_var("y", xval::valueof!(3_i64));
        s.set_var("label", xval::valueof!("result"));

        let tpl = Template::parse("@if (x > 5 && y < 10) {{{ label | upper }}: {{ x * y + 1 }}}")
            .unwrap();
        s.set_template("main", tpl);

        assert_eq!(s.render("main").unwrap(), "RESULT: 31");
    }

    #[test]
    fn full_page_template() {
        let mut s = scope();
        s.set_var("title", xval::valueof!("My Page"));
        s.set_var(
            "users",
            xval::valueof!((vec![xval::valueof!("alice"), xval::valueof!("bob"),])),
        );
        s.set_var("theme", xval::valueof!("dark"));

        s.set_template(
            "header",
            Template::parse("<h1>{{ title | upper }}</h1>").unwrap(),
        );
        s.set_template(
            "page",
            Template::parse(concat!(
                "@include('header')",
                "@match (theme) { 'dark' => {<body class='dark'>}, 'light' => {<body class='light'>}, _ => {<body>} }",
                "@for (user of users; track user) {",
                "@if (user == 'alice') {<b>{{ user }}</b>}",
                "@else{<span>{{ user }}</span>}",
                "}",
                "</body>",
            ))
            .unwrap(),
        );

        assert_eq!(
            s.render("page").unwrap(),
            "<h1>MY PAGE</h1><body class='dark'><b>alice</b><span>bob</span></body>"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip() {
        let src = "Hello {{ name | upper }}!";
        let tpl = Template::parse(src).unwrap();
        let json = serde_json::to_string(&tpl).unwrap();
        assert_eq!(json, format!("\"{src}\""));
        let tpl2: Template = serde_json::from_str(&json).unwrap();
        assert_eq!(tpl, tpl2);
    }

    /// Parse + render with an empty scope.
    fn render(src: &str) -> String {
        let tpl = Template::parse(src).unwrap();
        tpl.render(&Scope::new()).unwrap()
    }

    /// Parse + render, expecting an eval error.
    fn render_err(src: &str, scope: &Scope) -> ast::EvalError {
        let tpl = Template::parse(src).unwrap();
        tpl.render(scope).unwrap_err()
    }

    // ── Literals ────────────────────────────────────────────────────

    #[test]
    fn literal_null() {
        assert_eq!(render("{{ null }}"), "<null>");
    }

    #[test]
    fn literal_bool() {
        assert_eq!(render("{{ true }}"), "true");
        assert_eq!(render("{{ false }}"), "false");
    }

    #[test]
    fn literal_int() {
        assert_eq!(render("{{ 42 }}"), "42");
    }

    #[test]
    fn literal_float() {
        assert_eq!(render("{{ 3.14 }}"), "3.14");
    }

    #[test]
    fn literal_string() {
        assert_eq!(render("{{ 'hello' }}"), "hello");
    }

    // ── Arithmetic & operators ──────────────────────────────────────

    #[test]
    fn add_ints() {
        assert_eq!(render("{{ 2 + 3 }}"), "5");
    }

    #[test]
    fn float_promotion() {
        assert_eq!(render("{{ 1 + 2.5 }}"), "3.5");
    }

    #[test]
    fn string_concat() {
        assert_eq!(render("{{ 'a' + 'b' }}"), "ab");
    }

    #[test]
    fn division_by_zero() {
        let err = render_err("{{ 10 / 0 }}", &Scope::new());
        assert!(matches!(err.inner(), ast::EvalError::DivisionByZero(_)));
    }

    #[test]
    fn comparison() {
        assert_eq!(render("{{ 1 < 2 }}"), "true");
    }

    #[test]
    fn equality() {
        assert_eq!(render("{{ 'a' == 'a' }}"), "true");
    }

    // ── Unary operators ─────────────────────────────────────────────

    #[test]
    fn unary_not() {
        assert_eq!(render("{{ !true }}"), "false");
    }

    #[test]
    fn unary_neg() {
        assert_eq!(render("{{ -5 }}"), "-5");
    }

    // ── Short-circuit logic ─────────────────────────────────────────

    #[test]
    fn and_short_circuit() {
        // `missing` is undefined — but short-circuit prevents the error.
        assert_eq!(render("{{ false && missing }}"), "false");
    }

    #[test]
    fn or_short_circuit() {
        assert_eq!(render("{{ true || missing }}"), "true");
    }

    // ── Error cases ─────────────────────────────────────────────────

    #[test]
    fn undefined_variable() {
        let err = render_err("{{ x }}", &Scope::new());
        assert!(matches!(err.inner(), ast::EvalError::UndefinedVariable(_)));
    }

    #[test]
    fn undefined_pipe() {
        let mut s = Scope::new();
        s.set_var("x", xval::valueof!("hi"));
        let err = render_err("{{ x | missing }}", &s);
        assert!(matches!(err.inner(), ast::EvalError::UndefinedPipe(_)));
    }

    #[test]
    fn for_not_iterable() {
        let mut s = Scope::new();
        s.set_var("items", xval::valueof!(42_i64));
        let err = render_err("@for (n of items; track n) {x}", &s);
        assert!(matches!(err.inner(), ast::EvalError::NotIterable(_)));
    }

    #[test]
    fn include_missing_template() {
        let err = render_err("@include('missing')", &Scope::new());
        assert!(matches!(err.inner(), ast::EvalError::UndefinedTemplate(_)));
    }

    #[test]
    fn include_non_string_name() {
        let mut s = Scope::new();
        s.set_var("name", xval::valueof!(42_i64));
        let err = render_err("@include(name)", &s);
        assert!(matches!(err.inner(), ast::EvalError::TypeError(_)));
    }

    // ── Simple control flow ─────────────────────────────────────────

    #[test]
    fn if_true() {
        assert_eq!(render("@if (true) {yes}"), "yes");
    }

    #[test]
    fn if_false() {
        assert_eq!(render("@if (false) {yes}"), "");
    }

    #[test]
    fn if_else() {
        assert_eq!(render("@if (false) {a}@else{b}"), "b");
    }

    #[test]
    fn for_loop() {
        let mut s = scope();
        s.set_var(
            "items",
            xval::valueof!((vec![xval::valueof!(1_i64), xval::valueof!(2_i64)])),
        );
        let tpl = Template::parse("@for (n of items; track n) {[{{ n }}]}").unwrap();
        s.set_template("main", tpl);
        assert_eq!(s.render("main").unwrap(), "[1][2]");
    }

    #[test]
    fn for_empty() {
        let mut s = scope();
        s.set_var("items", xval::valueof!((vec![] as Vec<xval::Value>)));
        let tpl = Template::parse("@for (n of items; track n) {x}").unwrap();
        s.set_template("main", tpl);
        assert_eq!(s.render("main").unwrap(), "");
    }

    #[test]
    fn match_default() {
        let mut s = Scope::new();
        s.set_var("x", xval::valueof!("b"));
        let tpl = Template::parse("@match (x) { 'a' => {A}, _ => {?} }").unwrap();
        s.set_template("main", tpl);
        assert_eq!(s.render("main").unwrap(), "?");
    }

    #[test]
    fn match_arm() {
        let mut s = Scope::new();
        s.set_var("x", xval::valueof!("a"));
        let tpl = Template::parse("@match (x) { 'a' => {A}, 'b' => {B} }").unwrap();
        s.set_template("main", tpl);
        assert_eq!(s.render("main").unwrap(), "A");
    }

    #[test]
    fn match_no_match() {
        let mut s = Scope::new();
        s.set_var("x", xval::valueof!("c"));
        let tpl = Template::parse("@match (x) { 'a' => {A} }").unwrap();
        s.set_template("main", tpl);
        assert_eq!(s.render("main").unwrap(), "");
    }

    #[test]
    fn plain_text() {
        assert_eq!(render("hello world"), "hello world");
    }

    #[test]
    fn interpolation_with_variable() {
        let mut s = Scope::new();
        s.set_var("x", xval::valueof!(10_i64));
        let tpl = Template::parse("{{ x }}").unwrap();
        s.set_template("main", tpl);
        assert_eq!(s.render("main").unwrap(), "10");
    }
}
