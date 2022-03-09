use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::rc::Rc;

use crate::types::macros::{object_from_impl, object_to_impl};
use crate::types::*;

/// A reference-counted pointer to a VM object.
/// This allows objects to be moved around by cloning the pointer (i.e onto the eval stack).
#[derive(Clone)]
pub struct ObjectPtr {
    object: Rc<RefCell<Object>>,
    pub type_: &'static Type,
}

impl ObjectPtr {
    fn new(o: Object, type_: &'static Type) -> Self {
        Self {
            object: Rc::new(RefCell::new(o)),
            type_,
        }
    }

    pub fn borrow(&self) -> Ref<'_, Object> {
        self.object.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Object> {
        self.object.borrow_mut()
    }
}

impl Debug for ObjectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectPtr")
            .field("object", &self.object)
            .field("type_", &self.type_.name)
            .finish()
    }
}

/// A VM object.
/// This directly owns the underlying value.
#[derive(Debug)]
pub enum Object {
    Boolean(Boolean),
    Float(Float),
    Integer(Integer),
    Nil(Nil),
    String(String),
    Array(Array),
    Struct(Struct),
    Native(Native),
}

impl Object {
    /// Convert this object into a pointer.
    pub fn ptr(self) -> ObjectPtr {
        let type_ = match &self {
            Self::Boolean(_) => &*BooleanType,
            Self::Float(_) => &*FloatType,
            Self::Integer(_) => &*IntegerType,
            Self::Nil(_) => &*NilType,
            Self::String(_) => &*StringType,
            Self::Array(_) => &*ArrayType,
            Self::Struct(_) => &*StructType,
            Self::Native(t) => t.0,
        };

        ObjectPtr::new(self, type_)
    }
}

object_to_impl!(
    value;

    Boolean: match *value {
        Object::Boolean(b) => Ok(b),
        _ => Err(TypeError::Undefined),
    },

    Float: match *value {
        Object::Float(f) => Ok(f),
        Object::Integer(i) => Ok(i as Float),
        _ => Err(TypeError::Undefined),
    },

    Integer: match *value {
        Object::Float(f) => Ok(f as i64),
        Object::Integer(i) => Ok(i),
        _ => Err(TypeError::Undefined),
    },

    String: match value {
        Object::Boolean(b) => Ok(b.to_string()),
        Object::Float(f) => Ok(f.to_string()),
        Object::Integer(i) => Ok(i.to_string()),
        Object::Nil(_) => Ok("nil".to_owned()),
        Object::String(s) => Ok(s.to_owned()),
        _ => Err(TypeError::Undefined),
    }
);

object_from_impl!(Boolean, Float, Integer, Nil, String);
