use crate::types::Type;

pub type Integer = i64;

pub struct IntegerType;

impl Type<Integer> for IntegerType {}
