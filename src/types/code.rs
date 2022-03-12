use crate::runtime::Bytecode;
use crate::typeobject;
use crate::types::Native;

pub struct Code {
    bc: Bytecode,
}

impl From<Code> for Native {
    fn from(c: Code) -> Self {
        Native::new(&*CodeType, c)
    }
}

typeobject!(
    pub CodeType {
        name: "Code",
        ..Default::default()
    }
);
