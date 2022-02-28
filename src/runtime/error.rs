#[derive(Debug)]
pub enum VMError {
    EndOfStack,
    EndOfEvalStack,
    InvalidArgs,
    NotEnoughArgs,
    UnknownOp,
}

pub type VMResult = Result<(), VMError>;
