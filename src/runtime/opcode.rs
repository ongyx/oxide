use strum::IntoStaticStr;

#[derive(PartialEq, Eq, Hash, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Opcode {
    // stack
    PushConst,
    Pop,
    Load,
    LoadGlobal,
    Store,
    StoreGlobal,
    Delete,
    DeleteGlobal,

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
