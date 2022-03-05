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
        let frame = stack.current_frame()?;

        match self {
            PushConst(value) => {
                frame.eval.push(value.clone());
                Ok(())
            }
            Pop => {
                frame.eval.pop().ok_or(StackError::EvalEnd)?;
                Ok(())
            }
            Load(idx) => {
                let name = &bc.locals[*idx];
                let value = frame
                    .vars
                    .get(name)
                    .ok_or_else(|| VMError::Undefined(name.to_owned()))?;
                frame.eval.push(value.clone());
                Ok(())
            }
            Store(idx) => {
                let name = &bc.locals[*idx];
                frame.vars.insert(
                    name.to_owned(),
                    frame.eval.pop().ok_or(StackError::EvalEnd)?,
                );
                Ok(())
            }
            _ => Err(VMError::Unimplemented),
        }
    }
}
