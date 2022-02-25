use crate::ast::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Body(Vec<Node<'a>>),
    Loop {
        init: Option<Box<Node<'a>>>,
        cond: Box<Node<'a>>,
        next: Option<Box<Node<'a>>>,
        body: Box<Node<'a>>,
    },
    Function {
        name: Token<'a>,
        params: Vec<Token<'a>>,
        body: Box<Node<'a>>,
    },
    Return(Box<Node<'a>>),
    Assign {
        targets: Vec<Token<'a>>,
        expr: Box<Node<'a>>,
    },

    Array(Vec<Node<'a>>),

    Binop {
        op: Token<'a>,
        rhs: Box<Node<'a>>,
        lhs: Box<Node<'a>>,
    },
    Unop {
        op: Token<'a>,
        lhs: Box<Node<'a>>,
    },

    Value(Token<'a>),
}
