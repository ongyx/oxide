use strum::IntoStaticStr;

use crate::runtime::macros::{type_binop, type_fn, type_unop};
use crate::runtime::{Bytecode, Stack, VMError, VMResult};
use crate::types::{CmpOp, TypeError};

use Instruction::*;

/// A VM instruction.
#[derive(IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Instruction {
    // stack
    PushConst(usize),
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
    Cmp(CmpOp),
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
            PushConst(idx) => {
                local.push(bc.consts[*idx].clone());
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
            Add => type_binop!(local, add),
            Sub => type_binop!(local, sub),
            Mul => type_binop!(local, mul),
            Div => type_binop!(local, div),
            Pow => type_binop!(local, pow),
            Cmp(op) => {
                let rhs = local.pop()?;
                let lhs = local.pop()?;

                let cmp_fn = type_fn!(cmp(lhs, rhs))?;
                local.push(cmp_fn(lhs, rhs, *op)?);
            }
            And => type_binop!(local, and),
            Or => type_binop!(local, or),
            Not => type_unop!(local, not),
            _ => return Err(VMError::Unimplemented),
        };

        Ok(())
    }
}
