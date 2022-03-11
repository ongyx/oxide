use crate::types::{Native, ObjectPtr, Type};
use crate::{native_preamble, typeobject};

#[allow(unused)]
pub struct Struct {
    members: Vec<String>,
    values: Vec<ObjectPtr>,
}

impl Native for Struct {
    native_preamble!();

    fn type_(&self) -> &'static Type {
        &*StructType
    }
}

typeobject!(
    pub StructType {
        name: "struct",
        ..Default::default()
    }
);
