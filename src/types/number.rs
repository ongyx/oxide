use std::ops::{Add, Div, Mul, Sub};

use crate::object_impl;
use crate::types::Object;

trait Number:
    Sized
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + PartialEq
    + PartialOrd
{
}

pub type Integer = i64;
pub type Float = f64;

impl<T> Object for T
where
    T: Number,
{
    object_impl!(
        self,
        add(other) -> Self {
            Ok(self + other)
        },
        sub(other) -> Self {
            Ok(self - other)
        },
        mul(other) -> Self {
            Ok(self * other)
        },
        div(other) -> Self {
            Ok(self / other)
        },
        eq(other) -> bool {
            Ok(self == other)
        },
        le(other) -> bool {
            Ok(self <= other)
        },
        lt(other) -> bool {
            Ok(self < other)
        },
        ge(other) -> bool {
            Ok(self >= other)
        },
        gt(other) -> bool {
            Ok(self > other)
        },
        ne(other) -> bool {
            Ok(self != other)
        }
    );
}

impl Object for Integer {
    object_impl!(
        self,
        pow(other) -> Self {
            Ok(Integer::pow(self, other as u32))
        }
    );
}

impl Object for Float {
    object_impl!(
        self,
        pow(other) -> Self {
            Ok(Float::powf(self, other))
        }
    );
}
