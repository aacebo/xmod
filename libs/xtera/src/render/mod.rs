pub mod for_block;
pub mod if_block;
pub mod interp;
pub mod switch_block;

use crate::ast::{Node, NodeKind};
use crate::eval::{Context, Result};

pub fn render(nodes: &[Node], ctx: &mut Context) -> Result<String> {
    let mut output = String::new();
    for node in nodes {
        render_node(node, ctx, &mut output)?;
    }
    Ok(output)
}

pub fn render_node(node: &Node, ctx: &mut Context, output: &mut String) -> Result<()> {
    match &node.kind {
        NodeKind::Text(s) => output.push_str(s),
        NodeKind::Interp(expr) => interp::render_interp(expr, ctx, output)?,
        NodeKind::If(block) => if_block::render_if(block, ctx, output)?,
        NodeKind::For(block) => for_block::render_for(block, ctx, output)?,
        NodeKind::Switch(block) => switch_block::render_switch(block, ctx, output)?,
    }
    Ok(())
}

pub fn render_nodes_into(nodes: &[Node], ctx: &mut Context, output: &mut String) -> Result<()> {
    for node in nodes {
        render_node(node, ctx, output)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    fn render(src: &str, ctx: &mut Context) -> String {
        let template = parse::parse(src).unwrap();
        super::render(&template.nodes, ctx).unwrap()
    }

    #[test]
    fn plain_text() {
        let mut ctx = Context::new();
        assert_eq!(render("hello world", &mut ctx), "hello world");
    }

    #[test]
    fn interp() {
        let mut ctx = Context::new();
        ctx.set("name", xval::Value::from_str("Alice"));
        assert_eq!(render("Hello {{ name }}!", &mut ctx), "Hello Alice!");
    }

    #[test]
    fn interp_expr() {
        let mut ctx = Context::new();
        assert_eq!(render("{{ 1 + 2 }}", &mut ctx), "3");
    }

    #[test]
    fn if_truthy() {
        let mut ctx = Context::new();
        ctx.set("show", xval::Value::from_bool(true));
        assert_eq!(render("@if (show) {yes}", &mut ctx), "yes");
    }

    #[test]
    fn if_falsy() {
        let mut ctx = Context::new();
        ctx.set("show", xval::Value::from_bool(false));
        assert_eq!(render("@if (show) {yes}", &mut ctx), "");
    }

    #[test]
    fn if_else() {
        let mut ctx = Context::new();
        ctx.set("show", xval::Value::from_bool(false));
        assert_eq!(render("@if (show) {yes} @else {no}", &mut ctx), "no");
    }

    #[test]
    fn if_else_if() {
        let mut ctx = Context::new();
        ctx.set("a", xval::Value::from_bool(false));
        ctx.set("b", xval::Value::from_bool(true));
        assert_eq!(
            render("@if (a) {A} @else @if (b) {B} @else {C}", &mut ctx),
            "B"
        );
    }

    #[test]
    fn for_loop() {
        let mut ctx = Context::new();
        ctx.set(
            "items",
            xval::Value::from_array(vec![
                xval::Value::from_i64(1),
                xval::Value::from_i64(2),
                xval::Value::from_i64(3),
            ]),
        );
        assert_eq!(
            render("@for (x of items; track x) {{{ x }}}", &mut ctx),
            "123"
        );
    }

    #[test]
    fn for_loop_binding_does_not_leak() {
        let mut ctx = Context::new();
        ctx.set(
            "items",
            xval::Value::from_array(vec![xval::Value::from_i64(1)]),
        );
        let template = parse::parse("@for (x of items; track x) {{{ x }}}").unwrap();
        super::render(&template.nodes, &mut ctx).unwrap();
        assert!(ctx.get("x").is_none());
    }

    #[test]
    fn switch_case() {
        let mut ctx = Context::new();
        ctx.set("color", xval::Value::from_str("red"));
        assert_eq!(
            render(
                "@switch (color) { @case ('red') {RED} @case ('blue') {BLUE} @default {OTHER}}",
                &mut ctx
            ),
            "RED"
        );
    }

    #[test]
    fn switch_default() {
        let mut ctx = Context::new();
        ctx.set("color", xval::Value::from_str("green"));
        assert_eq!(
            render(
                "@switch (color) { @case ('red') {RED} @default {OTHER}}",
                &mut ctx
            ),
            "OTHER"
        );
    }

    #[test]
    fn pipe() {
        let mut ctx = Context::new();
        ctx.set("name", xval::Value::from_str("hello"));
        ctx.set_pipe("uppercase", |val, _args| {
            Ok(xval::Value::from_string(val.as_str().to_uppercase()))
        });
        assert_eq!(render("{{ name | uppercase }}", &mut ctx), "HELLO");
    }

    #[test]
    fn nested_for_if() {
        let mut ctx = Context::new();
        ctx.set(
            "items",
            xval::Value::from_array(vec![
                xval::Value::from_i64(1),
                xval::Value::from_i64(2),
                xval::Value::from_i64(3),
            ]),
        );
        assert_eq!(
            render(
                "@for (x of items; track x) {@if (x == 2) {found}}",
                &mut ctx
            ),
            "found"
        );
    }
}
