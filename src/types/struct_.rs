use lazy_static::lazy_static;

use crate::native_preamble;
use crate::types::{Native, ObjectPtr, Type};

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

lazy_static! {
    pub static ref StructType: Type = Type {
        name: "struct",
        ..Default::default()
    };
}
