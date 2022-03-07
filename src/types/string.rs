use crate::types::{Object, ObjectPtr, Type, TypeResult};

pub struct StringType;

impl Type for StringType {
    fn add(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let mut v = v.borrow().string()?;
        let w = w.borrow().string()?;

        v.push_str(&w);

        Ok(Object::from(v).ptr())
    }
}
