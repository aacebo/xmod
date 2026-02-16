mod error;
mod expr;
mod lexer;
mod node;
mod op;
mod parser;
mod span;
mod token;

pub use error::*;
pub use expr::*;
pub use lexer::*;
pub use node::*;
pub use op::*;
pub use parser::*;
pub use span::*;
pub use token::*;

/// Parse a template string into an AST.
pub fn parse(source: &str) -> Result<Template> {
    Parser::new(source).parse()
}
