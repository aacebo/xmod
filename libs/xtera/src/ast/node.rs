use super::eval::{EvalError, EvalErrorKind, Result, is_truthy};
use super::{Expr, Span};
use crate::Scope;

// ── Node enum ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Text(TextNode),
    Interp(InterpNode),
    If(IfNode),
    For(ForNode),
    Switch(SwitchNode),
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Self::Text(n) => n.span,
            Self::Interp(n) => n.span,
            Self::If(n) => n.span,
            Self::For(n) => n.span,
            Self::Switch(n) => n.span,
        }
    }

    pub fn render(&self, scope: &Scope) -> Result<String> {
        match self {
            Self::Text(n) => n.render(scope),
            Self::Interp(n) => n.render(scope),
            Self::If(n) => n.render(scope),
            Self::For(n) => n.render(scope),
            Self::Switch(n) => n.render(scope),
        }
    }
}

pub fn render_nodes(nodes: &[Node], scope: &Scope) -> Result<String> {
    let mut output = String::new();
    for node in nodes {
        output.push_str(&node.render(scope)?);
    }
    Ok(output)
}

// ── TextNode ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct TextNode {
    pub text: String,
    pub span: Span,
}

impl TextNode {
    pub fn render(&self, _scope: &Scope) -> Result<String> {
        Ok(self.text.clone())
    }
}

// ── InterpNode ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct InterpNode {
    pub expr: Expr,
    pub span: Span,
}

impl InterpNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let val = self.expr.eval(scope)?;
        Ok(val.to_string())
    }
}

// ── IfNode ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub branches: Vec<IfBranch>,
    pub else_body: Option<Vec<Node>>,
    pub span: Span,
}

impl IfNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        for branch in &self.branches {
            let cond = branch.condition.eval(scope)?;
            if is_truthy(&cond) {
                return render_nodes(&branch.body, scope);
            }
        }

        if let Some(else_body) = &self.else_body {
            return render_nodes(else_body, scope);
        }

        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}

// ── ForNode ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ForNode {
    pub binding: String,
    pub iterable: Expr,
    pub track: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}

impl ForNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let iterable = self.iterable.eval(scope)?;
        if !iterable.is_array() {
            return Err(EvalError::new(
                EvalErrorKind::NotIterable,
                self.iterable.span(),
            ));
        }

        let arr = iterable.as_array();
        let mut output = String::new();

        for item in arr.items() {
            let mut inner = scope.clone();
            inner.set_var(&self.binding, item.as_value());
            output.push_str(&render_nodes(&self.body, &inner)?);
        }

        Ok(output)
    }
}

// ── SwitchNode ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchNode {
    pub expr: Expr,
    pub cases: Vec<SwitchCase>,
    pub default: Option<Vec<Node>>,
    pub span: Span,
}

impl SwitchNode {
    pub fn render(&self, scope: &Scope) -> Result<String> {
        let expr_val = self.expr.eval(scope)?;

        for case in &self.cases {
            let case_val = case.value.eval(scope)?;
            if expr_val == case_val {
                return render_nodes(&case.body, scope);
            }
        }

        if let Some(default_body) = &self.default {
            return render_nodes(default_body, scope);
        }

        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub value: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}
