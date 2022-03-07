use crate::types::macros::{binop, unop};
use crate::types::ObjectPtr;

pub enum TypeError {
    Unimplemented,
}

pub type TypeResult<T> = Result<T, TypeError>;

#[allow(unused_variables)]
pub trait Type {
    binop!(add, sub, mul, div, pow, and, or);
    unop!(not);
}

pub trait ObjectType {
    fn type_() -> Box<dyn Type>;
}
