mod expr;
mod literal;
mod node;
mod stmt;

mod grammar;

#[cfg(test)]
mod test;

use peg::error::ParseError;
use peg::str::LineCol;

pub use expr::*;
pub use grammar::parser;
pub use literal::*;
pub use node::Node;
pub use stmt::*;

pub fn literal<'a, T>(value: T) -> ExprNode<'a>
where
    T: Into<Literal<'a>>,
{
    Expression::Literal(value.into()).node()
}

pub struct Ast<'a> {
    pub source: &'a str,
    pub root: Option<Body<'a>>,
}

impl<'a> Ast<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, root: None }
    }

    pub fn format(&self, e: ParseError<LineCol>) -> String {
        let code_line = self.source.lines().nth(e.location.line - 1).unwrap();

        format!(
            "
        error at line {line}:{column}
        {code}
        {ptr:>hint$}
        expected one of {tokens}
        ",
            line = e.location.line,
            column = e.location.column,
            code = code_line,
            ptr = "^",
            hint = e.location.column,
            tokens = e.expected
        )
    }

    pub fn parse(&mut self) -> Result<(), ParseError<LineCol>> {
        match parser::file(self.source) {
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
