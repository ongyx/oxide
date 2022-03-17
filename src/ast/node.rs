use std::fmt;

#[derive(Debug)]
pub struct Span(pub usize, pub usize);

#[derive(Debug)]
pub struct Node<T>
where
    T: fmt::Debug + PartialEq,
{
    pub value: T,
    pub location: Span,
}

impl<T> Node<T>
where
    T: fmt::Debug + PartialEq,
{
    pub fn new(value: T, location: Span) -> Self {
        Node { value, location }
    }
}

impl<T> PartialEq for Node<T>
where
    T: fmt::Debug + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        // NOTE: we probably don't want to compare the location
        // as it's only used for reporting errors.
        self.value == other.value
    }
}
