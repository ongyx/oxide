use crate::type_impl;
use crate::types::Type;

pub type Boolean = bool;

impl Type for Boolean {
    type_impl!(
        self,
        eq(other) -> bool {
            Ok(*self == other)
        },
        ne(other) -> bool {
            Ok(*self != other)
        },
        and(other) -> bool {
            Ok(*self && other)
        },
        or(other) -> bool {
            Ok(*self || other)
        }
    );
}
