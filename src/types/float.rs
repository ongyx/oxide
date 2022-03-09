use crate::types::macros::arith_impl;
use crate::types::{Object, ObjectPtr, Type, TypeResult};
use Object::*;

pub type Float = f64;

pub struct FloatType;

impl Type for FloatType {
    fn name(&self) -> &'static str {
        "float"
    }

    arith_impl!(
        Float;

        add: +,
        sub: -,
        mul: *,
        div: /
    );

    fn pow(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
        let v = Float::try_from(&*v.borrow())?;
        let w = &*w.borrow();

        if let Integer(i) = &*w {
            Ok(Object::from(v.powi(*i as i32)).ptr())
        } else {
            Ok(Object::from(v.powf(Float::try_from(w)?)).ptr())
        }
    }
}
