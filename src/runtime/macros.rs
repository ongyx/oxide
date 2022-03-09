macro_rules! type_fn {
    ($op:ident($lhs:ident, $rhs:ident)) => {
        $lhs.type_
            .$op
            .or($rhs.type_.$op)
            .ok_or(TypeError::Unimplemented)
    };
    ($op:ident($rhs:ident)) => {
        $rhs.type_.$op.ok_or(TypeError::Unimplemented)
    };
}

macro_rules! type_binop {
    ($frame:ident, $op:ident) => {{
        let rhs = $frame.pop()?;
        let lhs = $frame.pop()?;

        let func = type_fn!($op(lhs, rhs))?;
        $frame.push(func(lhs, rhs)?);
    }};
}

macro_rules! type_unop {
    ($frame:ident, $op:ident) => {{
        let rhs = $frame.pop()?;

        let func = type_fn!($op(rhs))?;
        $frame.push(func(rhs)?);
    }};
}

pub(crate) use {type_binop, type_fn, type_unop};
