use std::ops::{Add, Div, Mul, Sub};

use crate::types::{Object, ObjectResult};

trait Number:
    Sized + Add<Output = Self> + Div<Output = Self> + Mul<Output = Self> + Sub<Output = Self>
{
}

pub type Float = f64;
pub type Integer = i64;

impl<T> Object for T
where
    T: Number,
{
    fn add(self, other: Self) -> ObjectResult<Self> {
        Ok(self + other)
    }
    fn sub(self, other: Self) -> ObjectResult<Self> {
        Ok(self - other)
    }
    fn mul(self, other: Self) -> ObjectResult<Self> {
        Ok(self * other)
    }
    fn div(self, other: Self) -> ObjectResult<Self> {
        Ok(self / other)
    }
}

impl Object for Integer {
    fn pow(self, other: Self) -> ObjectResult<Self> {
        Ok(Integer::pow(self, other as u32))
    }
}

impl Object for Float {
    fn pow(self, other: Self) -> ObjectResult<Self> {
        Ok(Float::powf(self, other))
    }
}
