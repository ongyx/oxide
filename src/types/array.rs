use lazy_static::lazy_static;

use crate::types::{ObjectPtr, Type};

pub type Array = Vec<ObjectPtr>;

lazy_static! {
    pub static ref ArrayType: Type = Type {
        name: "array",
        ..Default::default()
    };
}
