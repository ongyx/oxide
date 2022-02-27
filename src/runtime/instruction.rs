use crate::runtime::{VMResult, VM};
use crate::types::Value;

/// Instruction is a VM operation associated with an opcode.
pub struct Instruction {
    /// The function to run on the VM.
    pub func: fn(&mut VM, &[Value]) -> VMResult,
}

impl Instruction {
    pub fn new(func: fn(&mut VM, &[Value]) -> VMResult) -> Self {
        Self { func }
    }
}
