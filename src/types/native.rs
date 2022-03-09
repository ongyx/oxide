use std::fmt;

use crate::types::Type;

pub struct Native(pub &'static Type);

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Native").field(&self.0.name).finish()
    }
}
