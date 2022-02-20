use std::fmt;

use crate::runtime::types::{Object, Type};

pub struct Boolean(bool);

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Object for Boolean {
    fn type_() -> Type {
        Type::Boolean
    }
}
