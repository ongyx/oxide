use lazy_static::lazy_static;

use crate::types::macros::arith;
use crate::types::{Object, Type};
use Object::*;

pub type Float = f64;

macro_rules! arith_f {
    ($op:tt) => {
        arith!(Float, $op)
    };
}

lazy_static! {
    pub static ref FloatType: Type = Type {
        name: "float",

        add: arith_f!(+),
        sub: arith_f!(-),
        mul: arith_f!(*),
        div: arith_f!(/),

        pow: Some(|v, w| {
            let v = Float::try_from(&*v.borrow())?;
            let w = &*w.borrow();

            if let Integer(i) = &*w {
                Ok(Object::from(v.powi(*i as i32)).ptr())
            } else {
                Ok(Object::from(v.powf(Float::try_from(w)?)).ptr())
            }
        }),

        ..Default::default()
    };
}
