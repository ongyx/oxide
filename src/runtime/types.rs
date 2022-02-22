mod boolean;
mod float;
mod integer;
mod nil;
mod object;
mod string;

pub use boolean::{Boolean, BooleanType};
pub use float::{Float, FloatType};
pub use integer::{Integer, IntegerType};
pub use nil::{Nil, NilType};
pub use object::{Object, ObjectType};
pub use string::{Str, StrType};

pub enum Types {
    Boolean(Boolean),
    Float(Float),
    Integer(Integer),
    Nil(Nil),
    Str(Str),
}
