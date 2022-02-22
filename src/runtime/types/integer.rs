use crate::runtime::types::{Object, ObjectType};

pub type IntegerType = i64;
pub type Integer = Object<IntegerType>;

impl ObjectType for IntegerType {
    fn add(&self, rhs: Self) -> Option<Self> {
        Some(*self + rhs)
    }

    fn sub(&self, rhs: Self) -> Option<Self> {
        Some(*self - rhs)
    }

    fn mul(&self, rhs: Self) -> Option<Self> {
        Some(*self * rhs)
    }

    fn div(&self, rhs: Self) -> Option<Self> {
        Some(*self / rhs)
    }
}
