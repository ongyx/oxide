pub type Id<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    Boolean(bool),
    Nil,
    Integer(i64),
    Float(f64),
    String(&'a str),
}

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

pub type Expr<'a> = Box<Expression<'a>>;
pub type Exprs<'a> = Vec<Expression<'a>>;

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
        name: Id<'a>,
        args: Exprs<'a>,
    },
}

impl<'a> From<Literal<'a>> for Expression<'a> {
    fn from(l: Literal<'a>) -> Self {
        Self::Literal(l)
    }
}

#[derive(Debug, PartialEq)]
pub struct Assign<'a> {
    pub targets: Vec<Id<'a>>,
    pub expr: Expression<'a>,
}

#[derive(Debug, PartialEq)]
pub struct AugAssign<'a> {
    pub target: Id<'a>,
    pub op: Op,
    pub expr: Expression<'a>,
}

#[derive(Debug, PartialEq)]
pub struct If<'a> {
    pub cond: Expression<'a>,
    pub body: Body<'a>,
}

#[derive(Debug, PartialEq)]
pub struct IfElse<'a> {
    pub if_: Vec<If<'a>>,
    pub else_: Option<Body<'a>>,
}

pub type Stmt<'a> = Box<Statement<'a>>;

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Loop {
        init: Option<Assign<'a>>,
        cond: Expression<'a>,
        next: Option<AugAssign<'a>>,
        body: Body<'a>,
    },

    Function {
        name: Id<'a>,
        params: Vec<Id<'a>>,
        body: Body<'a>,
    },

    IfElse(IfElse<'a>),

    Assign(Assign<'a>),
    AugAssign(AugAssign<'a>),

    Expr(Expression<'a>),

    // keywords
    Break,
    Continue,
    Import(Id<'a>),
    Return(Exprs<'a>),
}

impl<'a> From<IfElse<'a>> for Statement<'a> {
    fn from(ie: IfElse<'a>) -> Self {
        Self::IfElse(ie)
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

impl<'a> From<Expression<'a>> for Statement<'a> {
    fn from(e: Expression<'a>) -> Self {
        Self::Expr(e)
    }
}

pub type Body<'a> = Vec<Statement<'a>>;
