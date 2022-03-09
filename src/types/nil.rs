use lazy_static::lazy_static;

use crate::types::Type;

#[derive(Debug)]
pub struct Nil;

lazy_static! {
    pub static ref NilType: Type = Type {
        name: "nil",

        ..Default::default()
    };
}
