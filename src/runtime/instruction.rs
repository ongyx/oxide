use strum::IntoStaticStr;

use crate::types::CmpOp;

/// A VM instruction.
#[derive(IntoStaticStr, Debug)]
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
    Jump(usize),
    JumpIf(usize),
    JumpIfNot(usize),
    Func,
    Return,
    Call,
    Nop,
}
