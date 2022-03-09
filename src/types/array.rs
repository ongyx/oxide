use crate::types::{ObjectPtr, Type};

pub type Array = Vec<ObjectPtr>;

pub struct ArrayType;

impl Type for ArrayType {
    fn name(&self) -> &'static str {
        "array"
    }
}
