use lazy_static::lazy_static;

use crate::types::{Object, Type};

pub type Boolean = bool;

lazy_static! {
    pub static ref BooleanType: Type = Type {
        name: "bool",

        and: Some(|v, w| {
            let v = Boolean::try_from(&*v.borrow())?;
            let w = Boolean::try_from(&*w.borrow())?;

            Ok(Object::from(v && w).ptr())
        }),
        or: Some(|v, w| {
            let v = Boolean::try_from(&*v.borrow())?;
            let w = Boolean::try_from(&*w.borrow())?;

            Ok(Object::from(v || w).ptr())
        }),
        not: Some(|v| {
            let v = Boolean::try_from(&*v.borrow())?;

            Ok(Object::from(!v).ptr())
        }),

        ..Default::default()
    };
}
