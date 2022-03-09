use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::types::{ObjectPtr, Type};

pub type Struct = HashMap<ObjectPtr, ObjectPtr>;

lazy_static! {
    pub static ref StructType: Type = Type {
        name: "struct",
        ..Default::default()
    };
}
