use std::collections::HashMap;

use crate::runtime::{ObjectPtr, StackError};

const STACK_SIZE: usize = u8::MAX as usize;

/// A VM stack frame.
pub struct Frame {
    /// Namespace for variables.
    pub vars: HashMap<String, ObjectPtr>,
    /// Evaluation stack for bytecode operations.
    pub eval: Vec<ObjectPtr>,
}

impl Frame {
    /// Create a new frame.
    pub fn new() -> Self {
        Frame {
            vars: HashMap::new(),
            eval: Vec::new(),
        }
    }
}

pub struct Stack(Vec<Frame>);

impl Stack {
    pub fn new() -> Self {
        let mut stack = Stack(Vec::with_capacity(STACK_SIZE));
        stack.push(Frame::new());
        stack
    }

    /// Push a new frame onto the stack.
    pub fn push(&mut self, frame: Frame) {
        self.0.push(frame)
    }

    /// Pop the frame at the top of the stack.
    pub fn pop(&mut self) -> Option<Frame> {
        self.0.pop()
    }

    /// Get the frame at the top of the stack.
    pub fn current_frame(&mut self) -> Result<&mut Frame, StackError> {
        self.0.last_mut().ok_or(StackError::End)
    }
}
