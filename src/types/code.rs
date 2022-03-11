use crate::runtime::Bytecode;
use crate::types::{Native, Type};
use crate::{native_preamble, typeobject};

pub type Code = Bytecode;

impl Native for Code {
    native_preamble!();

    fn type_(&self) -> &'static Type {
        &*CodeType
    }
}

typeobject!(
    pub CodeType {
        name: "Code",
        ..Default::default()
    }
);
