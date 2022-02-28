use crate::runtime::Value;

const STACK_SIZE: usize = u8::MAX as usize;

/// A VM stack frame.
pub struct Frame {
    /// The variables local to this stack frame.
    pub locals: Vec<Value>,
    /// Evaluation stack for bytecode operations.
    pub eval: Vec<Value>,
    /// Return address.
    /// If it is None, this frame is considered the 'global' stack frame.
    pub address: Option<usize>,
}

impl Frame {
    pub fn new(address: Option<usize>) -> Self {
        Frame {
            locals: Vec::new(),
            eval: Vec::new(),
            address,
        }
    }
}

pub struct Stack(Vec<Frame>);

impl Stack {
    pub fn new() -> Self {
        let mut stack = Stack(Vec::with_capacity(STACK_SIZE));
        stack.0.push(Frame::new(None));

        stack
    }

    /// Get the frame at the top of the stack.
    pub fn current_frame(&mut self) -> Option<&mut Frame> {
        self.0.last_mut()
    }
}
