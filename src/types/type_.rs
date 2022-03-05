use crate::types::ObjectPtr;

pub enum TypeError {
    Unimplemented,
}

pub type TypeResult = Result<ObjectPtr, TypeError>;

macro_rules! binop {
    ($($name:ident),* $(,)?) => {
        $(
            #[allow(unused_variables)]
            fn $name(v: T, w: ObjectPtr) -> TypeResult {
                Err(TypeError::Unimplemented)
            }
        )*
    };
}

macro_rules! unop {
    ($($name:ident),* $(,)?) => {
        $(
            #[allow(unused_variables)]
            fn $name(v: T) -> TypeResult {
                Err(TypeError::Unimplemented)
            }
        )*
    };
}

pub trait Type<T> {
    binop!(add, sub, mul, div, pow, iadd, isub, imul, idiv, ipow, eq, le, lt, ge, gt, ne, and, or);
    unop!(not);
}
