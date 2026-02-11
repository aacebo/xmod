mod float;
mod int;
mod uint;

pub use float::*;
pub use int::*;
pub use uint::*;

use crate::Value;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
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
            v => panic!("expected Float, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn as_int(&self) -> &Int {
        match self {
            Self::Int(v) => v,
            v => panic!("expected Int, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn as_uint(&self) -> &UInt {
        match self {
            Self::UInt(v) => v,
            v => panic!("expected UInt, received {}", std::any::type_name_of_val(v)),
        }
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::UInt(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let f = Number::Float(Float::F64(1.0));
        assert!(f.is_float());
        assert!(!f.is_int());
        assert!(!f.is_uint());

        assert!(Number::Int(Int::I32(1)).is_int());
        assert!(Number::UInt(UInt::U32(1)).is_uint());
    }

    #[test]
    fn as_float() {
        let n = Number::Float(Float::F64(3.14));
        assert_eq!(*n.as_float(), Float::F64(3.14));
    }

    #[test]
    fn as_int() {
        let n = Number::Int(Int::I32(42));
        assert_eq!(*n.as_int(), Int::I32(42));
    }

    #[test]
    fn as_uint() {
        let n = Number::UInt(UInt::U32(42));
        assert_eq!(*n.as_uint(), UInt::U32(42));
    }

    #[test]
    #[should_panic(expected = "expected Float")]
    fn as_float_panics_on_mismatch() {
        Number::Int(Int::I32(1)).as_float();
    }

    #[test]
    #[should_panic(expected = "expected Int")]
    fn as_int_panics_on_mismatch() {
        Number::Float(Float::F64(1.0)).as_int();
    }

    #[test]
    #[should_panic(expected = "expected UInt")]
    fn as_uint_panics_on_mismatch() {
        Number::Int(Int::I32(1)).as_uint();
    }

    #[test]
    fn into_value() {
        let v = Value::from(Number::Int(Int::I32(5)));
        assert!(matches!(v, Value::Number(_)));
    }

    #[test]
    fn display() {
        assert_eq!(Number::Float(Float::F64(1.5)).to_string(), "1.5");
        assert_eq!(Number::Int(Int::I32(42)).to_string(), "42");
        assert_eq!(Number::UInt(UInt::U64(100)).to_string(), "100");
    }
}
