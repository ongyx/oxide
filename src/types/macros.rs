macro_rules! binop {
    ($($name:ident),* $(,)?) => {
        $(
            fn $name(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
                Err(TypeError::Unimplemented)
            }
        )*
    };
}

macro_rules! unop {
    ($($name:ident),* $(,)?) => {
        $(
            fn $name(&self, v: ObjectPtr) -> TypeResult<ObjectPtr> {
                Err(TypeError::Unimplemented)
            }
        )*
    };
}

macro_rules! arith_impl {
    ($type:ident; $($name:ident: $op:tt),+) => {
        $(
            fn $name(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
                match (&*v.borrow(), &*w.borrow()) {
                    (Integer(vi), Float(wf)) => Ok(Object::from(*vi as f64 $op *wf).ptr()),
                    (Float(vf), Integer(wi)) => Ok(Object::from(*vf $op *wi as f64).ptr()),
                    (v @ _, w @ _) => {
                        let v = $type::try_from(v)?;
                        let w = $type::try_from(w)?;

                        Ok(Object::from(v $op w).ptr())
                    }
                }
            }
        )+
    };
}

macro_rules! object_to_impl {
    ($name:ident; $($type:ty: $body:expr),+) => {
        $(
            impl TryFrom<&Object> for $type {
                type Error = TypeError;

                fn try_from($name: &Object) -> Result<Self, Self::Error> {
                    $body
                }
            }
        )*
    };
}

macro_rules! object_from_impl {
    ($($variant:ident),+) => {
        $(
            impl From<$variant> for Object {
                fn from(v: $variant) -> Self {
                    Self::$variant(v)
                }
            }
        )*
        // special variant for &str
        impl From<&str> for Object {
            fn from(v: &str) -> Self {
                Self::String(v.into())
            }
        }
    };
}

pub(crate) use {arith_impl, binop, object_from_impl, object_to_impl, unop};
