use std::fmt;

use crate::runtime::types::{Object, Type};

pub struct Nil;

impl fmt::Display for Nil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nil")
    }
}

impl Object for Nil {
    fn type_() -> Type {
        Type::Nil
    }
}
