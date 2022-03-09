mod array;
mod boolean;
mod float;
mod integer;
mod macros;
mod native;
mod nil;
mod object;
mod string;
mod struct_;
mod type_;

#[cfg(test)]
mod test;

pub use array::{Array, ArrayType};
pub use boolean::{Boolean, BooleanType};
pub use float::{Float, FloatType};
pub use integer::{Integer, IntegerType};
pub use native::Native;
pub use nil::{Nil, NilType};
pub use object::{Object, ObjectPtr};
pub use string::StringType;
pub use struct_::{Struct, StructType};
pub use type_::{CmpOp, Type, TypeError, TypeResult};

/// Create an ObjectPtr from a value.
#[macro_export]
macro_rules! obj {
    ($x:expr) => {
        Object::from($x).ptr()
    };
}

/// Create a Vec of values, converted to ObjectPtr.
#[macro_export]
macro_rules! ovec {
    () => {
        vec![]
    };
    ($elem:expr; $n:expr) => {
        vec![crate::obj!($elem), $n]
    };
    ($($x:expr),+ $(,)?) => {
        vec![$(crate::obj!($x)),+]
    };
}
