use crate::runtime::{Bytecode, Stack, VMResult};

pub struct VM {
    pub stack: Stack,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Stack::new(),
        }
    }
    pub fn execute(&mut self, bc: Bytecode) -> VMResult {
        bc.execute(&mut self.stack)?;
        Ok(())
    }
}
