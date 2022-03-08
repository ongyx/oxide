use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::macros::{object_from_impl, object_to_impl};
use crate::types::{Boolean, Float, Integer, Native, Nil, TypeError};

/// A reference-counted pointer to a VM object.
/// This allows objects to be moved around by cloning the pointer (i.e onto the eval stack).
#[derive(Clone, Debug)]
pub struct ObjectPtr(Rc<RefCell<Object>>);

impl ObjectPtr {
    fn new(o: Object) -> Self {
        Self(Rc::new(RefCell::new(o)))
    }

    pub fn borrow(&self) -> Ref<'_, Object> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Object> {
        self.0.borrow_mut()
    }
}

/// A VM object.
#[derive(Debug)]
pub enum Object {
    Boolean(Boolean),
    Float(Float),
    Integer(Integer),
    Nil(Nil),
    String(String),
    Array(Vec<ObjectPtr>),
    Struct(HashMap<Object, ObjectPtr>),
    Native(Native),
}

impl Object {
    /// Convert this object into a pointer.
    pub fn ptr(self) -> ObjectPtr {
        ObjectPtr::new(self)
    }
}

object_to_impl!(
    value;

    Boolean: match *value {
        Object::Boolean(b) => Ok(b),
        _ => Err(TypeError::Unimplemented),
    },

    Float: match *value {
        Object::Float(f) => Ok(f),
        Object::Integer(i) => Ok(i as Float),
        _ => Err(TypeError::Unimplemented),
    },

    Integer: match *value {
        Object::Float(f) => Ok(f as i64),
        Object::Integer(i) => Ok(i),
        _ => Err(TypeError::Unimplemented),
    },

    String: match value {
        Object::Boolean(b) => Ok(b.to_string()),
        Object::Float(f) => Ok(f.to_string()),
        Object::Integer(i) => Ok(i.to_string()),
        Object::Nil(_) => Ok("nil".to_owned()),
        Object::String(s) => Ok(s.to_owned()),
        _ => Err(TypeError::Unimplemented),
    }
);

object_from_impl!(Boolean, Float, Integer, Nil, String);
