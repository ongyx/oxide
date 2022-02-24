pub enum ObjectError {
    Unimplemented,
}

pub type ObjectResult<T> = Result<T, ObjectError>;

macro_rules! object_method {
    ($($method_name:ident),+) => {
        $(
            #[allow(unused_variables)]
            fn $method_name(self, other: Self) -> ObjectResult<Self> {
                Err(ObjectError::Unimplemented)
            }
        )+
    };
}

pub trait Object: Sized {
    object_method!(add, sub, mul, div, pow);
}
