use lazy_static::lazy_static;

use crate::types::{Object, Type};

lazy_static! {
    pub static ref StringType: Type = Type {
        name: "str",

        add: Some(|v, w| {
            let mut v = String::try_from(&*v.borrow())?;
            let w = String::try_from(&*w.borrow())?;

            v.push_str(&w);

            Ok(Object::from(v).ptr())
        }),

        ..Default::default()
    };
}
