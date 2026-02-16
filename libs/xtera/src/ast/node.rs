use super::{Expr, Span};

/// A parsed template — a sequence of nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    pub nodes: Vec<Node>,
    pub span: Span,
}

/// A single top-level template node.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

impl Node {
    pub fn new(kind: NodeKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    /// Raw text between control structures / interpolations.
    Text(String),

    /// `{{ expr }}`.
    Interpolation(Expr),

    /// `@if (cond) { body } @else if (cond) { body } @else { body }`.
    If(IfBlock),

    /// `@for (item of items; track item.id) { body }`.
    For(ForBlock),

    /// `@switch (expr) { @case (v) { ... } @default { ... } }`.
    Switch(SwitchBlock),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBlock {
    pub branches: Vec<IfBranch>,
    pub else_body: Option<Vec<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfBranch {
    pub condition: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForBlock {
    pub binding: String,
    pub iterable: Expr,
    pub track: Expr,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchBlock {
    pub expr: Expr,
    pub cases: Vec<SwitchCase>,
    pub default: Option<Vec<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub value: Expr,
    pub body: Vec<Node>,
    pub span: Span,
}
