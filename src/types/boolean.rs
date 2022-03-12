use crate::typeobject;
use crate::types::Object;

pub type Boolean = bool;

typeobject!(
    pub BooleanType {
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
    }
);
