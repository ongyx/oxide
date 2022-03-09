use crate::types::macros::{binop, unop};
use crate::types::ObjectPtr;

#[derive(Debug)]
pub enum TypeError {
    Unimplemented,
    Undefined,
}

pub type TypeResult<T> = Result<T, TypeError>;

#[allow(unused_variables)]
pub trait Type {
    fn name(&self) -> &'static str;

    binop!(add, sub, mul, div, pow, and, or);
    unop!(not);
}
