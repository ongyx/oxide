use std::collections::HashMap;

use crate::runtime::{Instruction, Opcode, VMError};

pub struct Table(HashMap<Opcode, Instruction>);

macro_rules! create_table {
    ($($op:ident($func:expr)),+) => {
        vec![
            $(
                (
                    Opcode::$op,
                    Instruction::new($func)
                ),
            )+
        ]
    };
}

impl Table {
    pub fn new() -> Self {
        let table: HashMap<_, _> = create_table!(
            PushConst(|vm, args| {
                let value = args.get(0).ok_or(VMError::NotEnoughArgs)?;
                vm.stack.push(value.clone());
                Ok(())
            }),
            Pop(|vm, _| {
                vm.stack.pop().ok_or(VMError::EndOfStack)?;
                Ok(())
            })
        )
        .into_iter()
        .collect();

        Table(table)
    }

    pub fn lookup<'a>(&'a self, op: &Opcode) -> Option<&'a Instruction> {
        self.0.get(op)
    }
}
