pub mod for_block;
pub mod if_block;
pub mod interp;
pub mod switch_block;

use crate::Scope;
use crate::ast::{Node, NodeKind};
use crate::eval::Result;

pub fn render(nodes: &[Node], ctx: &Scope) -> Result<String> {
    let mut output = String::new();
    for node in nodes {
        render_node(node, ctx, &mut output)?;
    }
    Ok(output)
}

pub fn render_node(node: &Node, ctx: &Scope, output: &mut String) -> Result<()> {
    match &node.kind {
        NodeKind::Text(s) => output.push_str(s),
        NodeKind::Interp(expr) => interp::render_interp(expr, ctx, output)?,
        NodeKind::If(block) => if_block::render_if(block, ctx, output)?,
        NodeKind::For(block) => for_block::render_for(block, ctx, output)?,
        NodeKind::Switch(block) => switch_block::render_switch(block, ctx, output)?,
    }
    Ok(())
}

pub fn render_nodes_into(nodes: &[Node], ctx: &Scope, output: &mut String) -> Result<()> {
    for node in nodes {
        render_node(node, ctx, output)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    fn render(src: &str, ctx: &Scope) -> String {
        let template = parse::parse(src).unwrap();
        template.render(ctx).unwrap()
    }

    #[test]
    fn plain_text() {
        let ctx = Scope::new();
        assert_eq!(render("hello world", &ctx), "hello world");
    }

    #[test]
    fn interp() {
        let mut ctx = Scope::new();
        ctx.set_var("name", xval::Value::from_str("Alice"));
        assert_eq!(render("Hello {{ name }}!", &ctx), "Hello Alice!");
    }

    #[test]
    fn interp_expr() {
        let ctx = Scope::new();
        assert_eq!(render("{{ 1 + 2 }}", &ctx), "3");
    }

    #[test]
    fn if_truthy() {
        let mut ctx = Scope::new();
        ctx.set_var("show", xval::Value::from_bool(true));
        assert_eq!(render("@if (show) {yes}", &ctx), "yes");
    }

    #[test]
    fn if_falsy() {
        let mut ctx = Scope::new();
        ctx.set_var("show", xval::Value::from_bool(false));
        assert_eq!(render("@if (show) {yes}", &ctx), "");
    }

    #[test]
    fn if_else() {
        let mut ctx = Scope::new();
        ctx.set_var("show", xval::Value::from_bool(false));
        assert_eq!(render("@if (show) {yes} @else {no}", &ctx), "no");
    }

    #[test]
    fn if_else_if() {
        let mut ctx = Scope::new();
        ctx.set_var("a", xval::Value::from_bool(false));
        ctx.set_var("b", xval::Value::from_bool(true));
        assert_eq!(render("@if (a) {A} @else @if (b) {B} @else {C}", &ctx), "B");
    }

    #[test]
    fn for_loop() {
        let mut ctx = Scope::new();
        ctx.set_var(
            "items",
            xval::Value::from_array(vec![
                xval::Value::from_i64(1),
                xval::Value::from_i64(2),
                xval::Value::from_i64(3),
            ]),
        );
        assert_eq!(render("@for (x of items; track x) {{{ x }}}", &ctx), "123");
    }

    #[test]
    fn for_loop_binding_does_not_leak() {
        let mut ctx = Scope::new();
        ctx.set_var(
            "items",
            xval::Value::from_array(vec![xval::Value::from_i64(1)]),
        );
        let template = parse::parse("@for (x of items; track x) {{{ x }}}").unwrap();
        template.render(&ctx).unwrap();
        assert!(ctx.var("x").is_none());
    }

    #[test]
    fn switch_case() {
        let mut ctx = Scope::new();
        ctx.set_var("color", xval::Value::from_str("red"));
        assert_eq!(
            render(
                "@switch (color) { @case ('red') {RED} @case ('blue') {BLUE} @default {OTHER}}",
                &ctx
            ),
            "RED"
        );
    }

    #[test]
    fn switch_default() {
        let mut ctx = Scope::new();
        ctx.set_var("color", xval::Value::from_str("green"));
        assert_eq!(
            render(
                "@switch (color) { @case ('red') {RED} @default {OTHER}}",
                &ctx
            ),
            "OTHER"
        );
    }

    struct UppercasePipe;
    impl crate::Pipe for UppercasePipe {
        fn invoke(
            &self,
            val: &xval::Value,
            _args: &[xval::Value],
        ) -> crate::eval::Result<xval::Value> {
            Ok(xval::Value::from_string(val.as_str().to_uppercase()))
        }
    }

    #[test]
    fn pipe() {
        let mut ctx = Scope::new();
        ctx.set_var("name", xval::Value::from_str("hello"));
        ctx.set_pipe("uppercase", UppercasePipe);
        assert_eq!(render("{{ name | uppercase }}", &ctx), "HELLO");
    }

    #[test]
    fn nested_for_if() {
        let mut ctx = Scope::new();
        ctx.set_var(
            "items",
            xval::Value::from_array(vec![
                xval::Value::from_i64(1),
                xval::Value::from_i64(2),
                xval::Value::from_i64(3),
            ]),
        );
        assert_eq!(
            render("@for (x of items; track x) {@if (x == 2) {found}}", &ctx),
            "found"
        );
    }
}
