use crate::runtime::types::{Object, ObjectType};

pub type BooleanType = bool;
pub type Boolean = Object<BooleanType>;

impl ObjectType for BooleanType {}
