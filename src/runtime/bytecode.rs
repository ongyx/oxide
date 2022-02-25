pub enum Opcode {
    // stack
    PushConst = 0,
    Pop = 1,
    Load = 2,
    LoadGlobal = 3,
    Store = 4,
    StoreGlobal = 5,
    Delete = 6,
    DeleteGlobal = 7,

    // binop/unop
    Add = 16,
    Sub = 17,
    Mul = 18,
    Div = 19,
    Pow = 20,
    Eq = 21,
    Le = 22,
    Lt = 23,
    Ge = 24,
    Gt = 25,
    Ne = 26,
    And = 27,
    Or = 28,
    Not = 29,

    // jump/function
    Jump = 32,
    JumpIf = 33,
    JumpIfNot = 34,
    Func = 35,
    Return = 36,
    Call = 37,
}
