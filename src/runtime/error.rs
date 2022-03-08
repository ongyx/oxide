#[derive(Debug)]
pub enum VMError {
    Unimplemented,
    Stack(StackError),
}

impl From<StackError> for VMError {
    fn from(error: StackError) -> Self {
        Self::Stack(error)
    }
}

pub type VMResult = Result<(), VMError>;

#[derive(Debug)]
pub enum StackError {
    End,
    EvalEnd,
    Undefined(String),
}
