use crate::types::Type;

#[derive(Debug)]
pub struct Nil;

pub struct NilType;

impl Type for NilType {
    fn name(&self) -> &'static str {
        "nil"
    }
}
