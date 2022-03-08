use crate::types::Type;

/// A wrapper around a native Rust type.
pub type Native = Box<dyn Type>;
