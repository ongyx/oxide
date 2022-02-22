use crate::runtime::types::{Object, ObjectType};

pub type FloatType = f64;
pub type Float = Object<FloatType>;

impl ObjectType for FloatType {
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
