mod boolean;
mod nil;
mod number;
mod object;
mod string;

use std::collections::HashMap;

pub use boolean::Boolean;
pub use nil::Nil;
pub use number::{Float, Integer};
pub use object::{Object, ObjectError, ObjectResult};
pub use string::Str;

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(Boolean),
    Float(Float),
    Integer(Integer),
    Nil(Nil),
    Str(Str),
    Array(Vec<Value>),
    Struct(HashMap<Value, Value>),
}
