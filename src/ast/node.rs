use crate::ast::token::Token;

// these are the same type but have different semantic meaning.
pub type Body<'a> = Vec<Node<'a>>;
pub type Exprs<'a> = Vec<Node<'a>>;

pub type BoxedNode<'a> = Box<Node<'a>>;

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Loop {
        init: Option<BoxedNode<'a>>,
        cond: BoxedNode<'a>,
        next: Option<BoxedNode<'a>>,
        body: Body<'a>,
    },

    Function {
        name: Token<'a>,
        params: Vec<Token<'a>>,
        body: Body<'a>,
    },
    Call {
        name: Token<'a>,
        args: Exprs<'a>,
    },
    Return(BoxedNode<'a>),

    If {
        if_: Vec<(BoxedNode<'a>, Body<'a>)>,
        else_: Option<Body<'a>>,
    },

    Keyword(Token<'a>),

    Assign {
        targets: Vec<Token<'a>>,
        expr: BoxedNode<'a>,
    },
    AugAssign {
        target: Token<'a>,
        op: Token<'a>,
        expr: BoxedNode<'a>,
    },

    Array(Exprs<'a>),

    Binop {
        op: Token<'a>,
        lhs: BoxedNode<'a>,
        rhs: BoxedNode<'a>,
    },
    Unop {
        op: Token<'a>,
        rhs: BoxedNode<'a>,
    },

    Value(Token<'a>),
}
