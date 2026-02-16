mod error;
pub mod lexer;
pub mod parser;
pub mod token;

pub use error::*;

/// Parse a template string into an AST.
pub fn parse(source: &str) -> Result<crate::Template> {
    parser::Parser::new(source).parse()
}
