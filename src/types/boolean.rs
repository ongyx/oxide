use crate::object_impl;
use crate::types::Object;

pub type Boolean = bool;

impl Object for Boolean {
    object_impl!(
        self,
        eq(other) -> bool {
            Ok(self == other)
        },
        ne(other) -> bool {
            Ok(self != other)
        },
        and(other) -> bool {
            Ok(self && other)
        },
        or(other) -> bool {
            Ok(self || other)
        }
    );
}
