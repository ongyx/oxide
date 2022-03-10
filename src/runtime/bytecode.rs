use crate::runtime::macros::{type_binop, type_fn, type_unop};
use crate::runtime::{Instruction, Stack, VMError, VMResult};
use crate::types::{Boolean, ObjectPtr, TypeError};

use Instruction::*;

/// A chunk of executable instructions.
pub struct Bytecode {
    /// Compiled instructions.
    pub code: Vec<Instruction>,
    /// Index of the instruction being executed.
    pub counter: usize,
    /// Local variable names.
    pub locals: Vec<String>,
    /// Constants.
    pub consts: Vec<ObjectPtr>,
}

impl Bytecode {
    pub fn new(code: Vec<Instruction>) -> Self {
        Self {
            code,
            counter: 0,
            locals: Vec::new(),
            consts: Vec::new(),
        }
    }

    /// Execute all the instructions.
    pub fn execute(&mut self, stack: &mut Stack) -> VMResult {
        let limit = self.code.len();

        loop {
            self.eval(stack)?;
            if self.counter == limit {
                break;
            }
        }

        Ok(())
    }

    /// Evaluate the instruction indexed by counter.
    pub fn eval(&mut self, stack: &mut Stack) -> VMResult {
        let (global, local) = stack.frames()?;
        let local = local.unwrap_or(global);

        let mut increment = true;

        let ins = &self.code[self.counter];
        println!("{:?}", ins);

        // TODO: return VMError instead of panicking on out-of-bounds?
        match ins {
            PushConst(idx) => {
                local.push(self.consts[*idx].clone());
            }
            Pop => {
                local.pop()?;
            }
            Load(idx) => {
                local.push(local.get(&self.locals[*idx])?);
            }
            LoadGlobal(s) => global.eval.push(global.get(s)?),
            Store(idx) => {
                let o = local.pop()?;
                local.set(self.locals[*idx].to_owned(), o);
            }
            StoreGlobal(s) => {
                let o = global.pop()?;
                global.set(s.to_owned(), o);
            }
            Delete(idx) => {
                local.delete(&self.locals[*idx])?;
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
            Nop => {}
            Jump(delta) => {
                increment = false;

                self.counter = *delta;
            }
            JumpIf(delta) => {
                increment = false;

                let v = local.pop()?;
                let b = Boolean::try_from(&*v.borrow())?;

                if b {
                    self.counter = *delta;
                }
            }
            JumpIfNot(delta) => {
                increment = false;

                let v = local.pop()?;
                let b = Boolean::try_from(&*v.borrow())?;

                if !b {
                    self.counter = *delta;
                }
            }
            _ => return Err(VMError::Unimplemented),
        };

        if increment {
            self.counter += 1;
        }

        Ok(())
    }
}
