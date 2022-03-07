use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::types::{Boolean, Float, Integer, Nil, TypeError, TypeResult};

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
    String(String),
    Array(Vec<ObjectPtr>),
    Struct(HashMap<Object, ObjectPtr>),
    // Native(Native),
}

impl Object {
    /// Convert this object into a pointer.
    pub fn ptr(self) -> ObjectPtr {
        Rc::new(RefCell::new(self))
    }

    pub fn boolean(&self) -> TypeResult<Boolean> {
        match self {
            Self::Boolean(b) => Ok(*b),
            _ => Err(TypeError::Unimplemented),
        }
    }

    pub fn float(&self) -> TypeResult<Float> {
        match self {
            Self::Float(f) => Ok(*f),
            Self::Integer(i) => Ok(*i as Float),
            _ => Err(TypeError::Unimplemented),
        }
    }

    pub fn integer(&self) -> TypeResult<Integer> {
        match self {
            Self::Float(f) => Ok(*f as i64),
            Self::Integer(i) => Ok(*i),
            _ => Err(TypeError::Unimplemented),
        }
    }

    pub fn string(&self) -> TypeResult<String> {
        match self {
            Self::Boolean(b) => Ok(b.to_string()),
            Self::Float(f) => Ok(f.to_string()),
            Self::Integer(i) => Ok(i.to_string()),
            Self::Nil(_) => Ok("nil".to_owned()),
            Self::String(s) => Ok(s.to_owned()),
            _ => Err(TypeError::Unimplemented),
        }
    }
}

macro_rules! object_from_impl {
    ($($variant:ident),+) => {
        $(
            impl From<$variant> for Object {
                fn from(v: $variant) -> Self {
                    Self::$variant(v)
                }
            }
        )*
    };
}

object_from_impl!(Boolean, Float, Integer, Nil, String);
