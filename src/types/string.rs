use crate::types::{Object, ObjectPtr, Type, TypeResult};

impl Type for String {
    fn add(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let mut v = String::try_from(&*v.borrow())?;
        let w = String::try_from(&*w.borrow())?;

        v.push_str(&w);

        Ok(Object::from(v).ptr())
    }
}
