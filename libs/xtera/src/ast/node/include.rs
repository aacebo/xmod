use crate::Scope;
use crate::ast::{
    EvalError, Expr, IncludeDepthError, Result, Span, TypeError, UndefinedTemplateError,
    value_type_name,
};

const MAX_INCLUDE_DEPTH: usize = 64;

#[derive(Debug, Clone, PartialEq)]
pub struct IncludeNode {
    pub name: Expr,
    pub span: Span,
}

impl IncludeNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        if scope.depth() >= MAX_INCLUDE_DEPTH {
            return Err(EvalError::IncludeDepth(IncludeDepthError).with_span(self.span.clone()));
        }

        let name_val = self.name.eval(scope)?;
        if !name_val.is_string() {
            return Err(EvalError::TypeError(TypeError {
                expected: "string",
                got: value_type_name(&name_val),
            })
            .with_span(self.span.clone()));
        }

        let name = name_val.as_string().as_str();
        let tpl = scope.template(name).ok_or_else(|| {
            EvalError::UndefinedTemplate(UndefinedTemplateError {
                name: name.to_string(),
            })
            .with_span(self.span.clone())
        })?;

        scope.inc_depth();
        let result = tpl.render(scope);
        scope.dec_depth();
        result
    }
}

impl std::fmt::Display for IncludeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
