use super::{BinaryOp, Span, UnaryOp};

/// An expression node with source span.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// `null`, `true`, `false`, number, or string.
    Literal(Literal),

    /// A bare identifier: `foo`.
    Ident(String),

    /// Member access: `a.b`.
    Member { object: Box<Expr>, field: String },

    /// Index access: `a[0]`.
    Index { object: Box<Expr>, index: Box<Expr> },

    /// Function call: `fn(arg1, arg2)`.
    Call { callee: Box<Expr>, args: Vec<Expr> },

    /// Pipe expression: `expr | name` or `expr | name:arg1:arg2`.
    Pipe {
        value: Box<Expr>,
        name: String,
        args: Vec<Expr>,
    },

    /// Binary operation: `a + b`.
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    /// Unary operation: `!a`, `-a`.
    Unary { op: UnaryOp, operand: Box<Expr> },

    /// Array literal: `[1, 2, 3]`.
    Array(Vec<Expr>),

    /// Object literal: `{ a: 1, b: 'two' }`.
    Object(Vec<(String, Expr)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "\"{v}\""),
        }
    }
}
