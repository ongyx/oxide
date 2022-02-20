use std::fmt;

use crate::runtime::types::{Object, Type};

pub struct Float(f64);

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Object for Float {
    fn type_() -> Type {
        Type::Float
    }
}
