pub mod node;
pub mod token;

mod parser;

#[cfg(test)]
mod test;

use logos::Span;
use peg::error::ParseError;

pub use crate::ast::node::{Body, Node};
pub use crate::ast::parser::oxide_parser as Parser;
pub use crate::ast::token::{tokenise, Token};

pub struct Ast<'a> {
    pub tokens: Vec<Token<'a>>,
    pub spans: Vec<Span>,
    pub root: Option<Body<'a>>,
    pub err: Option<ParseError<usize>>,
}

impl<'a> Ast<'a> {
    pub fn new(code: &str) -> Ast {
        let (tokens, spans) = tokenise(code);

        match Parser::body(&tokens) {
            Ok(node) => Ast {
                tokens,
                spans,
                root: Some(node),
                err: None,
            },
            Err(e) => Ast {
                tokens,
                spans,
                root: None,
                err: Some(e),
            },
        }
    }
}
