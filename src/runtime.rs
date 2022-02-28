mod bytecode;
mod error;
mod instruction;
mod opcode;
mod stack;
mod table;
mod value;
mod vm;

#[cfg(test)]
mod test;

pub use bytecode::Bytecode;
pub use error::{VMError, VMResult};
pub use instruction::Instruction;
pub use opcode::Opcode;
pub use stack::{Frame, Stack};
pub use table::Table;
pub use value::Value;
pub use vm::VM;
