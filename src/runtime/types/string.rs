use std::fmt;

use crate::runtime::types::{Object, Type};

pub struct Str(String);

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Object for Str {
    fn type_() -> Type {
        Type::Str
    }
}
