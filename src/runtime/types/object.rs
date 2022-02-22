#[allow(unused_variables)]
pub trait ObjectType: Sized {
    fn add(&self, rhs: Self) -> Option<Self> {
        None
    }
    fn sub(&self, rhs: Self) -> Option<Self> {
        None
    }
    fn mul(&self, rhs: Self) -> Option<Self> {
        None
    }
    fn div(&self, rhs: Self) -> Option<Self> {
        None
    }
}

pub struct Object<T>
where
    T: ObjectType,
{
    value: T,
}
