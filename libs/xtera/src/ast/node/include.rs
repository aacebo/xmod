use crate::Scope;
use crate::ast::{
    EvalError, Expr, Result, Span, TypeError, UndefinedTemplateError, value_type_name,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IncludeNode {
    pub name: Expr,
    pub span: Span,
}

impl IncludeNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let name_val = self.name.eval(scope)?;
        if !name_val.is_string() {
            return Err(EvalError::TypeError(TypeError {
                expected: "string",
                got: value_type_name(&name_val),
                span: self.span,
            }));
        }

        let name = name_val.as_string().as_str();
        let tpl = scope.template(name).ok_or_else(|| {
            EvalError::UndefinedTemplate(UndefinedTemplateError {
                name: name.to_string(),
                span: self.span,
            })
        })?;

        tpl.render(scope)
    }
}
