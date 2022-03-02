use crate::type_impl;
use crate::types::Type;

pub type Str = String;

impl Type for Str {
    type_impl!(
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
