use crate::ast::SwitchBlock;
use crate::eval::{Context, Result, eval_expr};

pub fn render_switch(
    switch_block: &SwitchBlock,
    ctx: &mut Context,
    output: &mut String,
) -> Result<()> {
    let expr_val = eval_expr(&switch_block.expr, ctx)?;

    for case in &switch_block.cases {
        let case_val = eval_expr(&case.value, ctx)?;
        if expr_val == case_val {
            super::render_nodes_into(&case.body, ctx, output)?;
            return Ok(());
        }
    }

    if let Some(default_body) = &switch_block.default {
        super::render_nodes_into(default_body, ctx, output)?;
    }

    Ok(())
}
