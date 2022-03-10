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

/// A type object.
#[derive(Default)]
pub struct Type {
    /// The type's name.
    pub name: &'static str,

    /// Arithmetic operations

    /// add(lhs, rhs)
    pub add: Option<Binop>,
    /// sub(lhs, rhs)
    pub sub: Option<Binop>,
    /// mul(lhs, rhs)
    pub mul: Option<Binop>,
    /// div(lhs, rhs)
    pub div: Option<Binop>,
    /// pow(lhs, rhs)
    pub pow: Option<Binop>,

    /// Boolean operations

    /// Boolean and.
    /// and(lhs, rhs)
    pub and: Option<Binop>,
    /// Boolean or.
    /// or(lhs, rhs)
    pub or: Option<Binop>,
    /// Boolean not.
    /// not(rhs)
    pub not: Option<Unop>,

    /// Compare lhs to rhs by cmpop.
    /// cmp(lhs, rhs, cmpop)
    pub cmp: Option<Cmp>,

    /// Call self with args.
    /// call(self, args)
    pub call: Option<Binop>,
}
