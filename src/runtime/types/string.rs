use crate::runtime::types::{Object, ObjectType};

pub type StrType = String;
pub type Str = Object<StrType>;

impl ObjectType for StrType {
    fn add(&self, rhs: Self) -> Option<Self> {
        let mut lhs = self.clone();
        lhs.push_str(&rhs);
        Some(lhs)
    }
}
