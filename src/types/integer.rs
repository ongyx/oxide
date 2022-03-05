use std::cmp::Ordering;

use crate::types::{Object, ObjectPtr, Type, TypeError, TypeResult};

use Object::*;

pub type Integer = i64;

pub struct IntegerType;

impl Type<Integer> for IntegerType {
    fn add(v: &mut Integer, w: ObjectPtr) -> TypeResult {
        match *w.borrow() {
            Integer(i) => Ok(Integer(*v + i).ptr()),
            Float(f) => Ok(Float(*v as f64 + f).ptr()),
            _ => Err(TypeError::Unimplemented),
        }
    }

    fn sub(v: &mut Integer, w: ObjectPtr) -> TypeResult {
        match *w.borrow() {
            Integer(i) => Ok(Integer(*v - i).ptr()),
            Float(f) => Ok(Float(*v as f64 - f).ptr()),
            _ => Err(TypeError::Unimplemented),
        }
    }

    fn mul(v: &mut Integer, w: ObjectPtr) -> TypeResult {
        match *w.borrow() {
            Integer(i) => Ok(Integer(*v * i).ptr()),
            Float(f) => Ok(Float(*v as f64 * f).ptr()),
            _ => Err(TypeError::Unimplemented),
        }
    }

    fn div(v: &mut Integer, w: ObjectPtr) -> TypeResult {
        let fw = match *w.borrow() {
            Integer(i) => i as f64,
            Float(f) => f,
            _ => return Err(TypeError::Unimplemented),
        };

        Ok(Float(*v as f64 / fw).ptr())
    }

    fn pow(v: &mut Integer, w: ObjectPtr) -> TypeResult {
        match *w.borrow() {
            Integer(i) => {
                if i >= 0 {
                    Ok(Integer(v.pow(i as u32)).ptr())
                } else {
                    Ok(Float((*v as f64).powi(i as i32)).ptr())
                }
            }
            Float(f) => Ok(Float((*v as f64).powf(f)).ptr()),
            _ => Err(TypeError::Unimplemented),
        }
    }

    fn cmp(v: &mut Integer, w: ObjectPtr) -> Option<Ordering> {
        match &*w.borrow() {
            Integer(i) => (*v).partial_cmp(i),
            Float(f) => (*v as f64).partial_cmp(f),
            _ => None,
        }
    }
}
