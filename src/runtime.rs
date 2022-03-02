mod bytecode;
mod error;
mod instruction;
mod object;
mod stack;
mod vm;

#[cfg(test)]
mod test;

pub use bytecode::Bytecode;
pub use error::{StackError, VMError, VMResult};
pub use instruction::Instruction;
pub use object::{Object, ObjectPtr};
pub use stack::{Frame, Stack};
pub use vm::VM;
