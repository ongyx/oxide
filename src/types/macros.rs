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
    ($type:ident) => {
        fn add(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
            let v = v.borrow().$type()?;
            let w = w.borrow().$type()?;

            Ok(Object::from(v + w).ptr())
        }

        fn sub(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
            let v = v.borrow().$type()?;
            let w = w.borrow().$type()?;

            Ok(Object::from(v - w).ptr())
        }

        fn mul(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
            let v = v.borrow().$type()?;
            let w = w.borrow().$type()?;

            Ok(Object::from(v * w).ptr())
        }

        fn div(&self, v: ObjectPtr, w: ObjectPtr) -> TypeResult<ObjectPtr> {
            let v = v.borrow().$type()?;
            let w = w.borrow().$type()?;

            Ok(Object::from(v / w).ptr())
        }
    };
}

pub(crate) use {arith_impl, binop, unop};
