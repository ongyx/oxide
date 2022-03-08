use std::collections::HashMap;

use crate::runtime::StackError;
use crate::types::ObjectPtr;

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

    /// Get a variable.
    /// This clones the ObjectPtr associated with it, so the reference count is incremented.
    pub fn get(&self, name: &String) -> Result<ObjectPtr, StackError> {
        match self.vars.get(name) {
            Some(o) => Ok(o.clone()),
            None => Err(StackError::Undefined(name.to_owned())),
        }
    }

    /// Set a ObjectPtr to a variable.
    /// This clones the ObjectPtr, so the reference count is incremented.
    pub fn set(&mut self, name: String, object: ObjectPtr) {
        self.vars.insert(name, object.clone());
    }

    /// Delete a variable.
    /// This destroys the ObjectPtr associated with it, so the reference count is decremented.
    pub fn delete(&mut self, name: &String) -> Result<(), StackError> {
        match self.vars.remove(name) {
            Some(_) => Ok(()),
            None => Err(StackError::Undefined(name.to_owned())),
        }
    }

    pub fn push(&mut self, o: ObjectPtr) {
        self.eval.push(o);
    }

    pub fn pop(&mut self) -> Result<ObjectPtr, StackError> {
        self.eval.pop().ok_or(StackError::EvalEnd)
    }
}

// NOTE: This compares frames by reference (i.e pointer addresses).
impl PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
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

    /// Get the global and local (current) frame.
    /// If the stack has only one frame, it will be treated as the global frame and the local frame will be None.
    pub fn frames(&mut self) -> Result<(&mut Frame, Option<&mut Frame>), StackError> {
        if self.len() == 0 {
            Err(StackError::End)
        } else {
            let (a, b) = self.0.split_at_mut(1);

            // should not panic because stack len must be more than 0
            let global = a.first_mut().expect("no global frame");
            let local = b.last_mut();

            Ok((global, local))
        }
    }

    /// Return how many frames are on the stack.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
