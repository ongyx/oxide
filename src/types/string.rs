use crate::typeobject;
use crate::types::{Object};

typeobject!(
    pub StringType {
        name: "str",

        add: Some(|v, w| {
            let mut v = String::try_from(&*v.borrow())?;
            let w = String::try_from(&*w.borrow())?;

            v.push_str(&w);

            Ok(Object::from(v).ptr())
        }),

        ..Default::default()
    }
);
