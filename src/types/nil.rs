use crate::typeobject;


#[derive(Debug)]
pub struct Nil;

typeobject!(
    pub NilType {
        name: "nil",
        ..Default::default()
    }
);
