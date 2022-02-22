use std::collections::HashMap;
use std::num::{ParseFloatError, ParseIntError};

pub enum ObjectError {
    Unimplemented,
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

#[derive(Debug)]
pub enum Object {
    Bool(bool),
    Float(f64),
    Int(i64),
    Nil,
    Str(String),
    Struct(HashMap<Object, Object>),
}

impl Object {
    pub fn to_str(&self) -> Result<String, ObjectError> {
        match self {
            Self::Bool(b) => Ok(b.to_string()),
            Self::Float(f) => Ok(f.to_string()),
            Self::Int(i) => Ok(i.to_string()),
            Self::Nil => Ok("nil".to_owned()),
            Self::Str(s) => Ok(s.to_owned()),
            _ => Err(ObjectError::Unimplemented),
        }
    }

    pub fn to_float(&self) -> Result<f64, ObjectError> {
        match self {
            Self::Float(f) => Ok(*f),
            Self::Int(i) => Ok(*i as f64),
            Self::Str(s) => Ok(s
                .to_owned()
                .parse::<f64>()
                .map_err(ObjectError::ParseFloat)?),
            _ => Err(ObjectError::Unimplemented),
        }
    }

    pub fn to_int(&self) -> Result<i64, ObjectError> {
        match self {
            Self::Float(f) => Ok(*f as i64),
            Self::Int(i) => Ok(*i),
            Self::Str(s) => Ok(s.to_owned().parse::<i64>().map_err(ObjectError::ParseInt)?),
            _ => Err(ObjectError::Unimplemented),
        }
    }

    pub fn add(&mut self, rhs: Self) -> Result<(), ObjectError> {
        match self {
            Self::Float(f) => {
                *f += rhs.to_float()?;
                Ok(())
            }
            Self::Int(i) => {
                *i += rhs.to_int()?;
                Ok(())
            }
            Self::Str(s) => {
                s.push_str(&rhs.to_str()?);
                Ok(())
            }
            _ => Err(ObjectError::Unimplemented),
        }
    }

    pub fn sub(&mut self, rhs: Self) -> Result<(), ObjectError> {
        match self {
            Self::Float(f) => {
                *f -= rhs.to_float()?;
                Ok(())
            }
            Self::Int(i) => {
                *i -= rhs.to_int()?;
                Ok(())
            }
            _ => Err(ObjectError::Unimplemented),
        }
    }

    pub fn mul(&mut self, rhs: Self) -> Result<(), ObjectError> {
        match self {
            Self::Float(f) => {
                *f *= rhs.to_float()?;
                Ok(())
            }
            Self::Int(i) => {
                *i *= rhs.to_int()?;
                Ok(())
            }
            _ => Err(ObjectError::Unimplemented),
        }
    }

    pub fn div(&mut self, rhs: Self) -> Result<(), ObjectError> {
        match self {
            Self::Float(f) => {
                *f /= rhs.to_float()?;
                Ok(())
            }
            Self::Int(i) => {
                *i /= rhs.to_int()?;
                Ok(())
            }
            _ => Err(ObjectError::Unimplemented),
        }
    }
}
