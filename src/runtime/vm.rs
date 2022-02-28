use crate::runtime::{Bytecode, Stack, Table, VMError, VMResult};

pub struct VM {
    pub stack: Stack,
    table: Table,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Stack::new(),
            table: Table::new(),
        }
    }

    pub fn execute(&mut self, bytecode: Vec<Bytecode>) -> VMResult {
        for bc in bytecode.iter() {
            let ins = self.table.lookup(bc.op).ok_or(VMError::UnknownOp)?;
            let frame = self.stack.current_frame().ok_or(VMError::EndOfStack)?;

            (ins.func)(frame, &bc.args)?;
        }

        Ok(())
    }
}
