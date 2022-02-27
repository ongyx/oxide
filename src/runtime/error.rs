#[derive(Debug)]
pub enum VMError {
    NotEnoughArgs,
    EndOfStack,
    UnknownOp,
}

pub type VMResult = Result<(), VMError>;
