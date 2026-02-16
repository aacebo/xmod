mod error;
mod lexer;
mod parser;
mod token;

pub use error::*;

/// Parse a template string into an AST.
pub fn parse(source: &str) -> Result<crate::ast::Template> {
    parser::Parser::new(source).parse()
}
