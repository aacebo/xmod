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

impl std::fmt::Display for IncludeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Template;
    use crate::ast::ValueExpr;

    fn val_expr(v: xval::Value) -> Expr {
        Expr::Value(ValueExpr {
            value: v,
            span: Span::new(0, 1),
        })
    }

    #[test]
    fn render_include() {
        let mut scope = Scope::new();
        scope.set_template("greeting", Template::parse("hello").unwrap());

        let node = IncludeNode {
            name: val_expr(xval::valueof!("greeting")),
            span: Span::new(0, 1),
        };

        assert_eq!(node.render(&scope).unwrap(), "hello");
    }

    #[test]
    fn render_missing_template() {
        let scope = Scope::new();
        let node = IncludeNode {
            name: val_expr(xval::valueof!("missing")),
            span: Span::new(0, 1),
        };

        let err = node.render(&scope).unwrap_err();
        assert!(matches!(err, EvalError::UndefinedTemplate(_)));
    }

    #[test]
    fn render_non_string_name() {
        let scope = Scope::new();
        let node = IncludeNode {
            name: val_expr(xval::valueof!(42_i64)),
            span: Span::new(0, 1),
        };

        let err = node.render(&scope).unwrap_err();
        assert!(matches!(err, EvalError::TypeError(_)));
    }
}
