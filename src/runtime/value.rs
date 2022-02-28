use std::collections::HashMap;

use crate::types;

/// A VM value.
#[derive(Clone, Debug)]
pub enum Value {
    Boolean(types::Boolean),
    Float(types::Float),
    Integer(types::Integer),
    Nil(types::Nil),
    Str(types::Str),
    Array(Vec<Value>),
    Struct(HashMap<Value, Value>),
}
