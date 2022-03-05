use std::cmp::Ordering;

use crate::types::ObjectPtr;

pub enum TypeError {
    Unimplemented,
}

pub type TypeResult = Result<ObjectPtr, TypeError>;

macro_rules! binop {
    ($($name:ident),* $(,)?) => {
        $(
            fn $name(v: &mut T, w: ObjectPtr) -> TypeResult {
                Err(TypeError::Unimplemented)
            }
        )*
    };
}

macro_rules! unop {
    ($($name:ident),* $(,)?) => {
        $(
            fn $name(v: &mut T) -> TypeResult {
                Err(TypeError::Unimplemented)
            }
        )*
    };
}

#[allow(unused_variables)]
pub trait Type<T> {
    binop!(add, sub, mul, div, pow, and, or);
    unop!(not);

    fn cmp(v: &mut T, w: ObjectPtr) -> Option<Ordering> {
        None
    }
}
