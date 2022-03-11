use crate::types::{Native, ObjectPtr, Type};
use crate::{native_preamble, typeobject};

/// An array of objects.
pub type Array = Vec<ObjectPtr>;

impl Native for Array {
    native_preamble!();

    fn type_(&self) -> &'static Type {
        &*ArrayType
    }
}

typeobject!(
    pub ArrayType {
        name: "array",
        ..Default::default()
    }
);
