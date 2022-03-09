use lazy_static::lazy_static;

use crate::types::macros::arith;
use crate::types::{Float, Object, Type};
use Object::*;

pub type Integer = i64;

macro_rules! arith_i {
    ($op:tt) => {
        arith!(Integer, $op)
    };
}

lazy_static! {
    pub static ref IntegerType: Type = Type {
        name: "int",

        add: arith_i!(+),
        sub: arith_i!(-),
        mul: arith_i!(*),
        div: arith_i!(/),

        pow: Some(|v, w| {
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
        }),

        ..Default::default()
    };
}
