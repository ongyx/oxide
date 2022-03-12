use crate::typeobject;
use crate::types::{Native, ObjectPtr};

#[allow(unused)]
pub struct Struct {
    members: Vec<String>,
    values: Vec<ObjectPtr>,
}

impl From<Struct> for Native {
    fn from(st: Struct) -> Self {
        Native::new(&*StructType, st)
    }
}

typeobject!(
    pub StructType {
        name: "struct",
        ..Default::default()
    }
);
