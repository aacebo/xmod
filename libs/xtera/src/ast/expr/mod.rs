mod array;
mod binary;
mod call;
mod ident;
mod index;
mod match_expr;
mod member;
mod object;
mod pipe;
mod unary;
mod value;

pub use array::*;
pub use binary::*;
pub use call::*;
pub use ident::*;
pub use index::*;
pub use match_expr::*;
pub use member::*;
pub use object::*;
pub use pipe::*;
pub use unary::*;
pub use value::*;

use super::{Result, Span};
use crate::Scope;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(ValueExpr),
    Ident(IdentExpr),
    Member(MemberExpr),
    Index(IndexExpr),
    Call(CallExpr),
    Pipe(PipeExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Array(ArrayExpr),
    Object(ObjectExpr),
    Match(MatchExpr),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Value(e) => e.span.clone(),
            Self::Ident(e) => e.span.clone(),
            Self::Member(e) => e.span.clone(),
            Self::Index(e) => e.span.clone(),
            Self::Call(e) => e.span.clone(),
            Self::Pipe(e) => e.span.clone(),
            Self::Binary(e) => e.span.clone(),
            Self::Unary(e) => e.span.clone(),
            Self::Array(e) => e.span.clone(),
            Self::Object(e) => e.span.clone(),
            Self::Match(e) => e.span.clone(),
        }
    }

    pub fn eval(&self, scope: &Scope) -> Result<xval::Value> {
        match self {
            Self::Value(e) => e.eval(scope),
            Self::Ident(e) => e.eval(scope),
            Self::Member(e) => e.eval(scope),
            Self::Index(e) => e.eval(scope),
            Self::Call(e) => e.eval(scope),
            Self::Pipe(e) => e.eval(scope),
            Self::Binary(e) => e.eval(scope),
            Self::Unary(e) => e.eval(scope),
            Self::Array(e) => e.eval(scope),
            Self::Object(e) => e.eval(scope),
            Self::Match(e) => e.eval(scope),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}
