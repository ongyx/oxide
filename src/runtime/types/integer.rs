use std::fmt;

use crate::runtime::types::{Object, Type};

pub struct Integer(i64);

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Object for Integer {
    fn type_() -> Type {
        Type::Integer
    }
}
