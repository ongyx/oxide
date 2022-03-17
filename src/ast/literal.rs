#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    Boolean(bool),
    Nil,
    Integer(i64),
    Float(f64),
    String(&'a str),
}

macro_rules! from_impl {
    ($($variant:ident($type:ty)),+) => {
        $(
            impl<'a> From<$type> for Literal<'a> {
                fn from(v: $type) -> Self {
                    Self::$variant(v)
                }
            }
        )+
    };
}

from_impl!(Boolean(bool), Integer(i64), Float(f64), String(&'a str));
