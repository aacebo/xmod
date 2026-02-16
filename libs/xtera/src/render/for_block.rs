use crate::Scope;
use crate::ast::ForBlock;
use crate::eval::{EvalError, EvalErrorKind, Result, eval_expr};

pub fn render_for(for_block: &ForBlock, scope: &Scope, output: &mut String) -> Result<()> {
    let iterable = eval_expr(&for_block.iterable, scope)?;
    if !iterable.is_array() {
        return Err(EvalError::new(
            EvalErrorKind::NotIterable,
            for_block.iterable.span,
        ));
    }

    let arr = iterable.as_array();

    for item in arr.items() {
        let mut inner = scope.clone();
        inner.set_var(&for_block.binding, item.as_value());
        super::render_nodes_into(&for_block.body, &inner, output)?;
    }

    Ok(())
}
