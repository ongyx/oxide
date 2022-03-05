use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::{Boolean, Float, Integer, Nil, Str};

/// A reference-counted pointer to a VM object.
/// This allows objects to be moved around by cloning the pointer (i.e onto the eval stack).
pub type ObjectPtr = Rc<RefCell<Object>>;

/// A VM object.
#[derive(Clone, Debug)]
pub enum Object {
    Boolean(Boolean),
    Float(Float),
    Integer(Integer),
    Nil(Nil),
    Str(Str),
    Array(Vec<ObjectPtr>),
    Struct(HashMap<Object, ObjectPtr>),
}

impl Object {
    /// Convert this object into a pointer.
    pub fn ptr(self) -> ObjectPtr {
        Rc::new(RefCell::new(self))
    }
}
