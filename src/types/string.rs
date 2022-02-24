use crate::types::{Object, ObjectResult};

pub type Str = String;

impl Object for Str {
    fn add(self, other: Self) -> ObjectResult<Self> {
        let mut s = self.clone();
        s.push_str(&other);
        Ok(s)
    }
}
