use std::any::Any;
use std::fmt;

use crate::types::Type;

pub trait Native {
    fn as_any(&self) -> &dyn Any;
    fn type_(&self) -> &'static Type;
}

impl fmt::Debug for Box<dyn Native> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Native")
            .field("type_", &self.type_().name)
            .finish()
    }
}

#[macro_export]
macro_rules! native_preamble {
    () => {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    };
}
