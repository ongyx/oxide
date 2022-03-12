use std::any::Any;
use std::fmt;

use crate::types::Type;

pub struct Native {
    pub type_: &'static Type,
    pub data: Box<dyn Any>,
}

impl Native {
    pub fn new<T: Any>(type_: &'static Type, data: T) -> Self {
        Native {
            type_,
            data: Box::new(data),
        }
    }
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Native")
            .field("type_", &self.type_.name)
            .field("data", &"<data>")
            .finish()
    }
}

pub trait _Native {}
impl<T> _Native for T {}
