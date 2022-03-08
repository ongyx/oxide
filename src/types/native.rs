use crate::types::Type;

#[derive(Debug)]
pub struct Native(pub &'static dyn Type);
