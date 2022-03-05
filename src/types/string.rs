use crate::types::{Object, ObjectPtr, Type, TypeError, TypeResult};

pub type Str = String;
pub struct StrType;

impl Type<Str> for StrType {
    fn add(v: Str, w: ObjectPtr) -> TypeResult {
        match &*w.borrow_mut() {
            Object::Str(s) => {
                let mut cv = v.clone();
                cv.push_str(&s);
                Ok(Object::Str(cv).ptr())
            }
            _ => Err(TypeError::Unimplemented),
        }
    }
}
