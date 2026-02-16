use crate::ast::{Node, NodeKind};

use super::context::Context;
use super::error::{EvalError, EvalErrorKind, Result};
use super::expr::{eval_expr, is_truthy};

pub fn render(nodes: &[Node], ctx: &mut Context) -> Result<String> {
    let mut output = String::new();
    for node in nodes {
        render_node(node, ctx, &mut output)?;
    }
    Ok(output)
}

fn render_node(node: &Node, ctx: &mut Context, output: &mut String) -> Result<()> {
    match &node.kind {
        NodeKind::Text(s) => {
            output.push_str(s);
        }
        NodeKind::Interpolation(expr) => {
            let val = eval_expr(expr, ctx)?;
            output.push_str(&val.to_string());
        }
        NodeKind::If(if_block) => {
            let mut matched = false;
            for branch in &if_block.branches {
                let cond = eval_expr(&branch.condition, ctx)?;
                if is_truthy(&cond) {
                    render_nodes_into(&branch.body, ctx, output)?;
                    matched = true;
                    break;
                }
            }
            if !matched {
                if let Some(else_body) = &if_block.else_body {
                    render_nodes_into(else_body, ctx, output)?;
                }
            }
        }
        NodeKind::For(for_block) => {
            let iterable = eval_expr(&for_block.iterable, ctx)?;
            if !iterable.is_array() {
                return Err(EvalError::new(
                    EvalErrorKind::NotIterable,
                    for_block.iterable.span,
                ));
            }
            let arr = iterable.as_array();

            // Save vars, iterate with binding, restore.
            let saved = ctx.child_scope();
            for item in arr.items() {
                let val = item.as_value();
                ctx.set(for_block.binding.clone(), val);
                render_nodes_into(&for_block.body, ctx, output)?;
            }
            ctx.with_vars(saved);
        }
        NodeKind::Switch(switch_block) => {
            let expr_val = eval_expr(&switch_block.expr, ctx)?;
            let mut matched = false;
            for case in &switch_block.cases {
                let case_val = eval_expr(&case.value, ctx)?;
                if expr_val == case_val {
                    render_nodes_into(&case.body, ctx, output)?;
                    matched = true;
                    break;
                }
            }
            if !matched {
                if let Some(default_body) = &switch_block.default {
                    render_nodes_into(default_body, ctx, output)?;
                }
            }
        }
    }
    Ok(())
}

fn render_nodes_into(nodes: &[Node], ctx: &mut Context, output: &mut String) -> Result<()> {
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
    fn interpolation() {
        let mut ctx = Context::new();
        ctx.set("name", xval::Value::from_str("Alice"));
        assert_eq!(render("Hello {{ name }}!", &mut ctx), "Hello Alice!");
    }

    #[test]
    fn interpolation_expr() {
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
