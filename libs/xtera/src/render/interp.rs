use crate::Scope;
use crate::ast::Expr;
use crate::eval::{Result, eval_expr};

pub fn render_interp(expr: &Expr, ctx: &Scope, output: &mut String) -> Result<()> {
    let val = eval_expr(expr, ctx)?;
    output.push_str(&val.to_string());
    Ok(())
}
