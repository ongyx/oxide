mod boolean;
mod nil;
mod number;
mod object;
mod string;

use std::collections::HashMap;

pub use boolean::Boolean;
pub use nil::Nil;
pub use number::{Float, Integer};
pub use object::{Object, ObjectResult};
pub use string::Str;

pub enum Primitive {
    Boolean(Boolean),
    Float(Float),
    Integer(Integer),
    Nil(Nil),
    Str(Str),
}

pub enum Value {
    Primitive(Primitive),
    Array(Vec<Value>),
    Struct(HashMap<Primitive, Value>),
}
