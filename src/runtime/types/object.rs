use std::fmt::Display;

pub enum Type {
    Boolean,
    Integer,
    Float,
    Str,
    Struct,
    Nil,
}

pub trait Object: Display {
    fn type_() -> Type;
}
