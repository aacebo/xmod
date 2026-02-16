use crate::Scope;
use crate::ast::IfBlock;
use crate::eval::{Result, eval_expr, is_truthy};

pub fn render_if(if_block: &IfBlock, ctx: &Scope, output: &mut String) -> Result<()> {
    for branch in &if_block.branches {
        let cond = eval_expr(&branch.condition, ctx)?;
        if is_truthy(&cond) {
            super::render_nodes_into(&branch.body, ctx, output)?;
            return Ok(());
        }
    }

    if let Some(else_body) = &if_block.else_body {
        super::render_nodes_into(else_body, ctx, output)?;
    }

    Ok(())
}
