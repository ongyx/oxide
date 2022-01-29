use crate::ast::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Body(Vec<Node<'a>>),
    Assign(Vec<Token<'a>>, Box<Node<'a>>),

    Array(Vec<Node<'a>>),

    Binop(Token<'a>, Box<Node<'a>>, Box<Node<'a>>),
    Unop(Token<'a>, Box<Node<'a>>),

    Atom(Token<'a>),
}
