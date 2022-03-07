use crate::types::macros::arith_impl;
use crate::types::{Object, ObjectPtr, Type, TypeResult};
use Object::*;

pub type Integer = i64;

pub struct IntegerType;

impl Type for IntegerType {
    arith_impl!(integer);

    fn pow(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = v.borrow().integer()?;
        let w = w.borrow();

        if let Integer(i) = &*w {
            let o = if *i >= 0 {
                Object::from(v.pow(*i as u32))
            } else {
                Object::from((v as f64).powi(*i as i32))
            };

            Ok(o.ptr())
        } else {
            Ok(Object::from((v as f64).powf(w.float()?)).ptr())
        }
    }
}
