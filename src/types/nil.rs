use crate::types::Type;

pub type Nil = ();
pub struct NilType;

impl Type<Nil> for NilType {}
