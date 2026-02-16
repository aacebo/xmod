use crate::ast::Expr;
use crate::eval::{Context, Result, eval_expr};

pub fn render_interp(expr: &Expr, ctx: &Context, output: &mut String) -> Result<()> {
    let val = eval_expr(expr, ctx)?;
    output.push_str(&val.to_string());
    Ok(())
}
