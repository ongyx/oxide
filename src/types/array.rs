use crate::typeobject;
use crate::types::{Native, ObjectPtr};

/// An array of objects.
#[allow(dead_code)]
pub struct Array {
    items: Vec<ObjectPtr>,
}

impl From<Array> for Native {
    fn from(arr: Array) -> Self {
        Native::new(&*ArrayType, arr)
    }
}

typeobject!(
    pub ArrayType {
        name: "array",
        ..Default::default()
    }
);
