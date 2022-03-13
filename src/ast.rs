pub mod node;

mod grammar;

#[cfg(test)]
mod test;

use peg::error::ParseError;
use peg::str::LineCol;

pub use crate::ast::grammar::parser;
use crate::ast::node::Body;

pub struct Ast<'a> {
    pub source: &'a str,
    pub root: Option<Body<'a>>,
}

impl<'a> Ast<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, root: None }
    }

    pub fn parse(&mut self) -> Result<(), ParseError<LineCol>> {
        match parser::body(self.source) {
            Ok(body) => {
                self.root = Some(body);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn parse_expr(&mut self) -> Result<(), ParseError<LineCol>> {
        match parser::expr(self.source) {
            Ok(body) => {
                self.root = Some(vec![body.into()]);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
