pub mod node;
pub mod token;

mod parser;

#[cfg(test)]
mod test;

use logos::Span;
use peg::error::ParseError;

use crate::ast::node::Node;
use crate::ast::parser::oxide_parser;
use crate::ast::token::{tokenise, Token};

pub struct Ast<'a> {
    pub tokens: Vec<Token<'a>>,
    pub spans: Vec<Span>,
    pub root: Option<Node<'a>>,
    pub err: Option<ParseError<usize>>,
}

impl<'a> Ast<'a> {
    pub fn new(code: &str) -> Ast {
        let (tokens, spans) = tokenise(code);

        match oxide_parser::body(&tokens) {
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
