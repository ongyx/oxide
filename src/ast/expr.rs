use crate::ast::{Literal, Node};

pub type Id<'a> = &'a str;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    // binop
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Le,
    Lt,
    Ge,
    Gt,
    Ne,
    And,
    Or,

    // unop
    Not,
}

pub type ExprNode<'a> = Node<Expression<'a>>;

pub type Expr<'a> = Box<ExprNode<'a>>;
pub type Exprs<'a> = Vec<ExprNode<'a>>;

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Id(Id<'a>),
    Literal(Literal<'a>),
    Array(Exprs<'a>),

    Binop {
        op: Op,
        lhs: Expr<'a>,
        rhs: Expr<'a>,
    },
    Unop {
        op: Op,
        rhs: Expr<'a>,
    },

    Call {
        value: Expr<'a>,
        args: Exprs<'a>,
    },

    Subscript {
        value: Expr<'a>,
        by: Expr<'a>,
    },

    Attribute {
        value: Expr<'a>,
        attr: Id<'a>,
    },
}

impl<'a> Expression<'a> {
    pub fn loc(self, start: usize, end: usize) -> ExprNode<'a> {
        Node::new(self, start, end)
    }
    pub fn node(self) -> ExprNode<'a> {
        self.loc(0, 0)
    }
}
