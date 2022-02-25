pub enum ObjectError {
    Unimplemented,
}

pub type ObjectResult<T> = Result<T, ObjectError>;

/*
#[macro_export]
macro_rules! object_impl {
    ($self:ident, $method:ident $(, $arg:ident)*, $body:block) => {
        fn $method($self $(, $arg: Self)*) -> $crate::types::ObjectResult<Self> {
            $body
        }
    };
}
*/

#[macro_export]
macro_rules! object_impl {
    ($self:ident, $($method:ident ($($arg:ident),*) -> $ret:ty $body:block),+) => {
        $(
            fn $method($self $(, $arg: Self)*) -> $crate::types::ObjectResult<$ret> {
                $body
            }
        )+
    };
}

macro_rules! object_method {
    ($($method:ident ($($arg:ident),*) -> $ret:ty),+) => {
        $(
            #[allow(unused_variables)]
            fn $method(self $(, $arg: Self)*) -> ObjectResult<$ret> {
                Err(ObjectError::Unimplemented)
            }
        )+
    };
}

pub trait Object: Sized {
    object_method!(
        add(other) -> Self,
        sub(other) -> Self,
        mul(other) -> Self,
        div(other) -> Self,
        pow(other) -> Self,
        eq(other) -> bool,
        le(other) -> bool,
        lt(other) -> bool,
        ge(other) -> bool,
        gt(other) -> bool,
        ne(other) -> bool,
        and(other) -> bool,
        or(other) -> bool,
        not() -> bool
    );
}
