use std::fmt;

use crate::types::macros::{binop, unop};
use crate::types::ObjectPtr;

#[derive(Debug)]
pub enum TypeError {
    Unimplemented,
}

pub type TypeResult<T> = Result<T, TypeError>;

#[allow(unused_variables)]
pub trait Type: fmt::Debug {
    binop!(add, sub, mul, div, pow, and, or);
    unop!(not);
}
