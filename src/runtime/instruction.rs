use strum::IntoStaticStr;

use crate::runtime::{Bytecode, Stack, VMError, VMResult};
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
                local.push(value.clone());
            }
            Pop => {
                local.pop()?;
            }
            Load(idx) => {
                local.push(local.get(&bc.locals[*idx])?);
            }
            LoadGlobal(s) => global.eval.push(global.get(s)?),
            Store(idx) => {
                let o = local.pop()?;
                local.set(bc.locals[*idx].to_owned(), o);
            }
            StoreGlobal(s) => {
                let o = global.pop()?;
                global.set(s.to_owned(), o);
            }
            Delete(idx) => {
                local.delete(&bc.locals[*idx])?;
            }
            DeleteGlobal(s) => {
                global.delete(s)?;
            }
            Add => {
                let rhs = local.pop()?;
                let lhs = local.pop()?;

                let v = lhs.borrow().type_().add(lhs.clone(), rhs.clone())?;
                local.push(v);
            }
            _ => return Err(VMError::Unimplemented),
        };

        Ok(())
    }
}
