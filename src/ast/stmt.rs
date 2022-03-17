use crate::ast::{ExprNode, Expression, Id, Node, Op, Span};

#[derive(Debug, PartialEq)]
pub struct Assign<'a> {
    pub targets: Vec<Id<'a>>,
    pub expr: ExprNode<'a>,
}

#[derive(Debug, PartialEq)]
pub struct AugAssign<'a> {
    pub target: Id<'a>,
    pub op: Op,
    pub expr: ExprNode<'a>,
}

#[derive(Debug, PartialEq)]
pub struct If<'a> {
    pub cond: ExprNode<'a>,
    pub body: Body<'a>,
}

#[derive(Debug, PartialEq)]
pub struct IfElse<'a> {
    pub chain: Vec<If<'a>>,
    pub else_: Option<Body<'a>>,
}

pub type StmtNode<'a> = Node<Statement<'a>>;

pub type Stmt<'a> = Box<StmtNode<'a>>;
pub type Body<'a> = Vec<StmtNode<'a>>;

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Iter {
        targets: Vec<Id<'a>>,
        value: ExprNode<'a>,
        body: Body<'a>,
    },

    Loop {
        init: Option<Assign<'a>>,
        cond: ExprNode<'a>,
        next: Option<AugAssign<'a>>,
        body: Body<'a>,
    },

    Function {
        name: Id<'a>,
        params: Vec<Id<'a>>,
        varargs: Option<Id<'a>>,
        body: Body<'a>,
    },

    Struct {
        name: Id<'a>,
        fields: Vec<Id<'a>>,
    },

    IfElse(IfElse<'a>),

    Assign(Assign<'a>),
    AugAssign(AugAssign<'a>),

    Expr(Expression<'a>),

    // keywords
    Break,
    Continue,
    Import {
        package: Id<'a>,
        members: Option<Vec<Id<'a>>>,
    },
    Return(Expression<'a>),
}

impl<'a> Statement<'a> {
    pub fn loc(self, start: usize, end: usize) -> StmtNode<'a> {
        Node::new(self, Span(start, end))
    }
    pub fn node(self) -> StmtNode<'a> {
        self.loc(0, 0)
    }
}

impl<'a> From<IfElse<'a>> for Statement<'a> {
    fn from(i: IfElse<'a>) -> Self {
        Self::IfElse(i)
    }
}

impl<'a> From<Assign<'a>> for Statement<'a> {
    fn from(a: Assign<'a>) -> Self {
        Self::Assign(a)
    }
}

impl<'a> From<AugAssign<'a>> for Statement<'a> {
    fn from(a: AugAssign<'a>) -> Self {
        Self::AugAssign(a)
    }
}

impl<'a> From<ExprNode<'a>> for StmtNode<'a> {
    fn from(e: ExprNode<'a>) -> Self {
        Node::new(Statement::Expr(e.value), e.location)
    }
}
