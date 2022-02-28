use crate::runtime::{Frame, VMResult, Value};

/// Instruction is a VM operation associated with an opcode.
pub struct Instruction {
    /// The function to run on the VM.
    pub func: fn(&mut Frame, &[Value]) -> VMResult,
}

impl Instruction {
    pub fn new(func: fn(&mut Frame, &[Value]) -> VMResult) -> Self {
        Self { func }
    }
}
