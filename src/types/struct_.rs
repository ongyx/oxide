use std::collections::HashMap;

use crate::types::{ObjectPtr, Type};

pub type Struct = HashMap<ObjectPtr, ObjectPtr>;

impl Type for Struct {}
