mod float;
mod int;
mod uint;

pub use float::*;
pub use int::*;
pub use uint::*;

use crate::Value;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Number {
    Float(Float),
    Int(Int),
    UInt(UInt),
}

impl Number {
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    pub fn is_uint(&self) -> bool {
        matches!(self, Self::UInt(_))
    }

    pub fn as_float(&self) -> &Float {
        match self {
            Self::Float(v) => v,
            v => panic!(
                "{}",
                format!("expected Float, received {}", std::any::type_name_of_val(v))
            ),
        }
    }

    pub fn as_int(&self) -> &Int {
        match self {
            Self::Int(v) => v,
            v => panic!(
                "{}",
                format!("expected Int, received {}", std::any::type_name_of_val(v))
            ),
        }
    }

    pub fn as_uint(&self) -> &UInt {
        match self {
            Self::UInt(v) => v,
            v => panic!(
                "{}",
                format!("expected UInt, received {}", std::any::type_name_of_val(v))
            ),
        }
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}
