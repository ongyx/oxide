use std::cmp::Ordering;

use crate::types::{Object, ObjectPtr, Type, TypeError, TypeResult};

use Object::*;

pub type Boolean = bool;
pub struct BooleanType;

impl Type<Boolean> for BooleanType {
    fn and(v: &mut Boolean, w: ObjectPtr) -> TypeResult {
        match *w.borrow() {
            Boolean(b) => Ok(Boolean(*v && b).ptr()),
            _ => Err(TypeError::Unimplemented),
        }
    }

    fn or(v: &mut Boolean, w: ObjectPtr) -> TypeResult {
        match *w.borrow() {
            Boolean(b) => Ok(Boolean(*v || b).ptr()),
            _ => Err(TypeError::Unimplemented),
        }
    }

    fn not(v: &mut Boolean) -> TypeResult {
        Ok(Boolean(!*v).ptr())
    }

    fn cmp(v: &mut Boolean, w: ObjectPtr) -> Option<Ordering> {
        match &*w.borrow() {
            Boolean(b) => (*v).partial_cmp(b),
            _ => None,
        }
    }
}
