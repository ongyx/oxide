use lazy_static::lazy_static;

use crate::native_preamble;
use crate::types::{Native, ObjectPtr, Type};

/// An array of objects.
pub type Array = Vec<ObjectPtr>;

impl Native for Array {
    native_preamble!();

    fn type_(&self) -> &'static Type {
        &*ArrayType
    }
}

lazy_static! {
    pub static ref ArrayType: Type = Type {
        name: "array",
        ..Default::default()
    };
}
