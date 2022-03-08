use crate::types::TypeError;

#[derive(Debug)]
pub enum VMError {
    Unimplemented,
    Stack(StackError),
    Type(TypeError),
}

impl From<StackError> for VMError {
    fn from(error: StackError) -> Self {
        Self::Stack(error)
    }
}

impl From<TypeError> for VMError {
    fn from(error: TypeError) -> Self {
        Self::Type(error)
    }
}

pub type VMResult = Result<(), VMError>;

#[derive(Debug)]
pub enum StackError {
    End,
    EvalEnd,
    Undefined(String),
}
