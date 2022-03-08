use strum::IntoStaticStr;

use crate::runtime::{Bytecode, Stack, StackError, VMError, VMResult};
use crate::types::ObjectPtr;

use Instruction::*;

/// A VM instruction.
#[derive(IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Instruction {
    // stack
    PushConst(ObjectPtr),
    Pop,
    Load(usize),
    LoadGlobal(String),
    Store(usize),
    StoreGlobal(String),
    Delete(usize),
    DeleteGlobal(String),

    // binop/unop
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    IAdd,
    ISub,
    IMul,
    IDiv,
    IPow,
    Eq,
    Le,
    Lt,
    Ge,
    Gt,
    Ne,
    And,
    Or,
    Not,

    // jump/function
    Jump,
    JumpIf,
    JumpIfNot,
    Func,
    Return,
    Call,
}

impl Instruction {
    pub fn run(&self, bc: &Bytecode, stack: &mut Stack) -> VMResult {
        let (global, local) = stack.frames()?;
        let local = local.unwrap_or(global);

        match self {
            PushConst(value) => {
                local.eval.push(value.clone());
                Ok(())
            }
            Pop => {
                local.eval.pop().ok_or(StackError::EvalEnd)?;
                Ok(())
            }
            Load(idx) => {
                let name = &bc.locals[*idx];
                let value = local.get(name)?;
                local.eval.push(value);
                Ok(())
            }
            LoadGlobal(s) => {
                let value = global.get(s)?;
                global.eval.push(value);
                Ok(())
            }
            Store(idx) => {
                let name = bc.locals[*idx].to_owned();
                let value = local.eval.pop().ok_or(StackError::EvalEnd)?;
                local.set(name, &value);
                Ok(())
            }
            _ => Err(VMError::Unimplemented),
        }
    }
}
