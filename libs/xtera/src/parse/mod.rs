mod error;
pub mod lexer;
pub mod parser;
pub mod token;

pub use error::*;

use std::sync::Arc;

/// Parse a template string into an AST.
pub fn parse(source: &str) -> Result<crate::Template> {
    let src: Arc<str> = Arc::from(source);
    parser::Parser::new(source, src).parse()
}
