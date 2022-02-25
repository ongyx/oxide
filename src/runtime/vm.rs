use crate::types::{Primitive, Value};

pub struct VM {
    stack: Vec<Value>,
}

impl VM {
    fn new() -> Self {
        VM { stack: Vec::new() }
    }
}
