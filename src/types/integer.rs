use crate::types::macros::arith_impl;
use crate::types::{Float, Object, ObjectPtr, Type, TypeResult};
use Object::*;

pub type Integer = i64;

pub struct IntegerType;

impl Type for IntegerType {
    arith_impl!(Integer);

    fn pow(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = Integer::try_from(&*v.borrow())?;
        let w = &*w.borrow();

        if let Integer(i) = &*w {
            let o = if *i >= 0 {
                Object::from(v.pow(*i as u32))
            } else {
                Object::from((v as f64).powi(*i as i32))
            };

            Ok(o.ptr())
        } else {
            Ok(Object::from((v as f64).powf(Float::try_from(w)?)).ptr())
        }
    }
}
