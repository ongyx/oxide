use crate::runtime::{Opcode, Value};

/// Bytecode represents a VM instruction to execute.
pub struct Bytecode {
    /// The opcode of the instruction.
    pub op: Opcode,
    /// The args to pass to the instruction.
    pub args: Vec<Value>,
}
