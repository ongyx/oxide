use crate::types::{Object, ObjectPtr, Type, TypeResult};

pub type Boolean = bool;
pub struct BooleanType;

impl Type for BooleanType {
    fn and(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = v.borrow().boolean()?;
        let w = w.borrow().boolean()?;

        Ok(Object::from(v && w).ptr())
    }

    fn or(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = v.borrow().boolean()?;
        let w = w.borrow().boolean()?;

        Ok(Object::from(v || w).ptr())
    }

    fn not(&self, v: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = v.borrow().boolean()?;

        Ok(Object::from(v).ptr())
    }
}
