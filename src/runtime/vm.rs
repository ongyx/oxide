use crate::runtime::{Bytecode, Table, VMError, VMResult};
use crate::types::Value;

pub struct VM {
    pub stack: Vec<Value>,
    table: Table,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            table: Table::new(),
        }
    }

    pub fn execute(&mut self, bytecode: Vec<Bytecode>) -> VMResult {
        for bc in bytecode.iter() {
            let func = self
                .table
                .lookup(&bc.op)
                .ok_or(VMError::UnknownOp)?
                .func
                .clone();

            (func)(self, &bc.args)?;
        }

        Ok(())
    }
}
