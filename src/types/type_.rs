pub enum TypeError {
    Unimplemented,
}

pub type TypeResult<T> = Result<T, TypeError>;

#[macro_export]
macro_rules! type_impl {
    ($self:ident, $($method:ident ($($arg:ident),*) -> $ret:ty $body:block),+) => {
        $(
            fn $method(&mut $self $(, $arg: Self)*) -> $crate::types::TypeResult<$ret> {
                $body
            }
        )+
    };
}

macro_rules! type_method {
    ($($method:ident ($($arg:ident),*) -> $ret:ty),+) => {
        $(
            #[allow(unused_variables)]
            fn $method(&mut self $(, $arg: Self)*) -> TypeResult<$ret> {
                Err(TypeError::Unimplemented)
            }
        )+
    };
}

pub trait Type: Sized {
    type_method!(
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
