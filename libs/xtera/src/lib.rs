pub mod ast;
pub mod parse;
mod scope;
mod template;

pub use scope::*;
pub use template::*;

#[cfg(feature = "derive")]
pub use xtera_derive::*;

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
}
