use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::object_impl;
use crate::types::Object;

trait Number:
    Sized
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + DivAssign
    + MulAssign
    + SubAssign
    + PartialEq
    + PartialOrd
    + Copy
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
            Ok(*self + other)
        },

        sub(other) -> Self {
            Ok(*self - other)
        },

        mul(other) -> Self {
            Ok(*self * other)
        },

        div(other) -> Self {
            Ok(*self / other)
        },

        iadd(other) -> () {
            *self += other;
            Ok(())
        },

        isub(other) -> () {
            *self -= other;
            Ok(())
        },

        imul(other) -> () {
            *self *= other;
            Ok(())
        },

        idiv(other) -> () {
            *self /= other;
            Ok(())
        },

        ipow(other) -> () {
            *self = self.pow(other)?;
            Ok(())
        },

        eq(other) -> bool {
            Ok(*self == other)
        },

        le(other) -> bool {
            Ok(*self <= other)
        },

        lt(other) -> bool {
            Ok(*self < other)
        },

        ge(other) -> bool {
            Ok(*self >= other)
        },

        gt(other) -> bool {
            Ok(*self > other)
        },

        ne(other) -> bool {
            Ok(*self != other)
        }
    );
}

impl Object for Integer {
    object_impl!(
        self,
        pow(other) -> Self {
            Ok(Integer::pow(*self, other as u32))
        }
    );
}

impl Object for Float {
    object_impl!(
        self,
        pow(other) -> Self {
            Ok(Float::powf(*self, other))
        }
    );
}
