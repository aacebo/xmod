use crate::ast::ForBlock;
use crate::eval::{Context, EvalError, EvalErrorKind, Result, eval_expr};

pub fn render_for(for_block: &ForBlock, ctx: &mut Context, output: &mut String) -> Result<()> {
    let iterable = eval_expr(&for_block.iterable, ctx)?;
    if !iterable.is_array() {
        return Err(EvalError::new(
            EvalErrorKind::NotIterable,
            for_block.iterable.span,
        ));
    }

    let arr = iterable.as_array();
    let saved = ctx.child_scope();

    for item in arr.items() {
        let val = item.as_value();
        ctx.set(for_block.binding.clone(), val);
        super::render_nodes_into(&for_block.body, ctx, output)?;
    }

    ctx.with_vars(saved);
    Ok(())
}
