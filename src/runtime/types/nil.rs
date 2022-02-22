use crate::runtime::types::{Object, ObjectType};

pub type NilType = ();
pub type Nil = Object<NilType>;

impl ObjectType for NilType {}
