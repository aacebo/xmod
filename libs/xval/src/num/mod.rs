mod float;
mod int;
mod uint;

pub use float::*;
pub use int::*;
pub use uint::*;

use crate::{ToValue, Value};

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

impl Number {
    pub fn is_f32(&self) -> bool {
        matches!(self, Self::Float(v) if v.is_f32())
    }

    pub fn is_f64(&self) -> bool {
        matches!(self, Self::Float(v) if v.is_f64())
    }

    pub fn to_f32(&self) -> f32 {
        self.as_float().to_f32()
    }

    pub fn to_f64(&self) -> f64 {
        self.as_float().to_f64()
    }
}

impl Number {
    pub fn is_i8(&self) -> bool {
        matches!(self, Self::Int(v) if v.is_i8())
    }

    pub fn is_i16(&self) -> bool {
        matches!(self, Self::Int(v) if v.is_i16())
    }

    pub fn is_i32(&self) -> bool {
        matches!(self, Self::Int(v) if v.is_i32())
    }

    pub fn is_i64(&self) -> bool {
        matches!(self, Self::Int(v) if v.is_i64())
    }

    pub fn to_i8(&self) -> i8 {
        self.as_int().to_i8()
    }

    pub fn to_i16(&self) -> i16 {
        self.as_int().to_i16()
    }

    pub fn to_i32(&self) -> i32 {
        self.as_int().to_i32()
    }

    pub fn to_i64(&self) -> i64 {
        self.as_int().to_i64()
    }
}

impl Number {
    pub fn is_u8(&self) -> bool {
        matches!(self, Self::UInt(v) if v.is_u8())
    }

    pub fn is_u16(&self) -> bool {
        matches!(self, Self::UInt(v) if v.is_u16())
    }

    pub fn is_u32(&self) -> bool {
        matches!(self, Self::UInt(v) if v.is_u32())
    }

    pub fn is_u64(&self) -> bool {
        matches!(self, Self::UInt(v) if v.is_u64())
    }

    pub fn to_u8(&self) -> u8 {
        self.as_uint().to_u8()
    }

    pub fn to_u16(&self) -> u16 {
        self.as_uint().to_u16()
    }

    pub fn to_u32(&self) -> u32 {
        self.as_uint().to_u32()
    }

    pub fn to_u64(&self) -> u64 {
        self.as_uint().to_u64()
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

impl ToValue for Number {
    fn to_value(self) -> Value {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let f = Number::from(1.0f64);
        assert!(f.is_float());
        assert!(!f.is_int());
        assert!(!f.is_uint());

        assert!(Number::from(1i32).is_int());
        assert!(Number::from(1u32).is_uint());
    }

    #[test]
    fn as_float() {
        let n = Number::from(3.14f64);
        assert_eq!(*n.as_float(), Float::F64(3.14));
    }

    #[test]
    fn as_int() {
        let n = Number::from(42i32);
        assert_eq!(*n.as_int(), Int::I32(42));
    }

    #[test]
    fn as_uint() {
        let n = Number::from(42u32);
        assert_eq!(*n.as_uint(), UInt::U32(42));
    }

    #[test]
    #[should_panic(expected = "expected Float")]
    fn as_float_panics_on_mismatch() {
        Number::from(1i32).as_float();
    }

    #[test]
    #[should_panic(expected = "expected Int")]
    fn as_int_panics_on_mismatch() {
        Number::from(1.0f64).as_int();
    }

    #[test]
    #[should_panic(expected = "expected UInt")]
    fn as_uint_panics_on_mismatch() {
        Number::from(1i32).as_uint();
    }

    #[test]
    fn into_value() {
        let v = Value::from(5i32);
        assert!(matches!(v, Value::Number(_)));
    }

    #[test]
    fn display() {
        assert_eq!(Number::from(1.5f64).to_string(), "1.5");
        assert_eq!(Number::from(42i32).to_string(), "42");
        assert_eq!(Number::from(100u64).to_string(), "100");
    }
}
