use crate::object_impl;
use crate::types::Object;

pub type Str = String;

impl Object for Str {
    object_impl!(
        self,
        add(other) -> Self {
            let mut s = self.clone();
            s.push_str(&other);
            Ok(s)
        },
        iadd(other) -> () {
            self.push_str(&other);
            Ok(())
        }
    );
}
