use crate::types::{Object, ObjectPtr, Type, TypeResult};

pub type Boolean = bool;

impl Type for Boolean {
    fn and(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = Boolean::try_from(&*v.borrow())?;
        let w = Boolean::try_from(&*w.borrow())?;

        Ok(Object::from(v && w).ptr())
    }

    fn or(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = Boolean::try_from(&*v.borrow())?;
        let w = Boolean::try_from(&*w.borrow())?;

        Ok(Object::from(v || w).ptr())
    }

    fn not(&self, v: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = Boolean::try_from(&*v.borrow())?;

        Ok(Object::from(!v).ptr())
    }
}
