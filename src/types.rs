use lazy_static::lazy_static;

mod macros;
mod native;
mod object;
mod type_;

pub use native::Native;
pub use object::{Object, ObjectPtr};
pub use type_::{CmpOp, Type, TypeError, TypeResult};

// primitive types
mod boolean;
mod float;
mod integer;
mod nil;
mod string;

pub use boolean::{Boolean, BooleanType};
pub use float::{Float, FloatType};
pub use integer::{Integer, IntegerType};
pub use nil::{Nil, NilType};
pub use string::StringType;

// native types
mod array;
mod code;
mod struct_;

pub use array::{Array, ArrayType};
pub use struct_::{Struct, StructType};

#[cfg(test)]
mod test;

/// Create an ObjectPtr from a value.
#[macro_export]
macro_rules! obj {
    ($x:expr) => {
        $crate::types::Object::from($x).ptr()
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

/// Create a new type object.
#[macro_export]
macro_rules! typeobject {
    (pub $name:ident $body:tt) => {
        $crate::types::lazy_static! {
            pub static ref $name: $crate::types::Type = $crate::types::Type $body;
        }
    };
    ($name:ident $body:tt) => {
        $crate::types::lazy_static! {
            static ref $name: $crate::types::Type = $crate::types::Type $body;
        }
    };
}
