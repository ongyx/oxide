use crate::runtime::{Instruction, Stack, VMResult};
use crate::types::ObjectPtr;

/// A chunk of executable instructions.
pub struct Bytecode {
    /// Compiled instructions.
    pub code: Vec<Instruction>,
    /// Local variable names.
    pub locals: Vec<String>,
    /// Constants.
    pub consts: Vec<ObjectPtr>,
}

impl Bytecode {
    pub fn new(code: Vec<Instruction>) -> Self {
        Self {
            code,
            locals: Vec::new(),
            consts: Vec::new(),
        }
    }

    pub fn execute(&self, stack: &mut Stack) -> VMResult {
        for ins in self.code.iter() {
            ins.run(self, stack)?;
        }

        Ok(())
    }
}
