use crate::types::ObjectPtr;

#[derive(Debug)]
pub enum TypeError {
    Unimplemented,
    Undefined,
}

pub type TypeResult<T> = Result<T, TypeError>;

type Binop = fn(ObjectPtr, ObjectPtr) -> TypeResult<ObjectPtr>;
type Unop = fn(ObjectPtr) -> TypeResult<ObjectPtr>;

#[derive(Clone, Copy, Debug)]
pub enum CmpOp {
    Eq,
    Le,
    Lt,
    Ge,
    Gt,
    Ne,
}

type Cmp = fn(ObjectPtr, ObjectPtr, CmpOp) -> TypeResult<ObjectPtr>;

#[derive(Default)]
pub struct Type {
    pub name: &'static str,

    pub add: Option<Binop>,
    pub sub: Option<Binop>,
    pub mul: Option<Binop>,
    pub div: Option<Binop>,
    pub pow: Option<Binop>,
    pub and: Option<Binop>,
    pub or: Option<Binop>,

    pub not: Option<Unop>,

    pub cmp: Option<Cmp>,
}
