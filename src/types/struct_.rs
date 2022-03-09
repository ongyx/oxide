use std::collections::HashMap;

use crate::types::{ObjectPtr, Type};

pub type Struct = HashMap<ObjectPtr, ObjectPtr>;

pub struct StructType;

impl Type for StructType {
    fn name(&self) -> &'static str {
        "struct"
    }
}
