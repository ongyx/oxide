pub enum ObjectError {
    Unimplemented,
}

pub type ObjectResult<T> = Result<T, ObjectError>;

#[macro_export]
macro_rules! object_impl {
    ($self:ident, $($method:ident ($($arg:ident),*) -> $ret:ty $body:block),+) => {
        $(
            fn $method(&mut $self $(, $arg: Self)*) -> $crate::types::ObjectResult<$ret> {
                $body
            }
        )+
    };
}

macro_rules! object_method {
    ($($method:ident ($($arg:ident),*) -> $ret:ty),+) => {
        $(
            #[allow(unused_variables)]
            fn $method(&mut self $(, $arg: Self)*) -> ObjectResult<$ret> {
                Err(ObjectError::Unimplemented)
            }
        )+
    };
}

pub trait Object: Sized {
    object_method!(
        // arithmetic
        add(other) -> Self,
        sub(other) -> Self,
        mul(other) -> Self,
        div(other) -> Self,
        pow(other) -> Self,

        // in-place arithmetic
        iadd(other) -> (),
        isub(other) -> (),
        imul(other) -> (),
        idiv(other) -> (),
        ipow(other) -> (),

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
