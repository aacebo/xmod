mod float;
mod int;
mod uint;

pub use float::*;
pub use int::*;
pub use uint::*;

use crate::{ToValue, Value};

/// A numeric value that can hold a float, signed integer, or unsigned integer.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum Number {
    Int(Int),
    UInt(UInt),
    Float(Float),
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

    pub fn as_float(&self) -> &Float {
        match self {
            Self::Float(v) => v,
            v => panic!("expected Float, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::Int(v) => v.type_id(),
            Self::UInt(v) => v.type_id(),
            Self::Float(v) => v.type_id(),
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

impl<'a> From<Number> for Value<'a> {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::UInt(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
        }
    }
}

impl ToValue<'_> for Number {
    fn to_value(self) -> Value<'static> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let f = Number::from_f64(1.0);
        assert!(f.is_float());
        assert!(!f.is_int());
        assert!(!f.is_uint());

        assert!(Number::from_i32(1).is_int());
        assert!(Number::from_u32(1).is_uint());
    }

    #[test]
    fn as_float() {
        let n = Number::from_f64(3.14);
        assert_eq!(*n.as_float(), Float::from_f64(3.14));
    }

    #[test]
    fn as_int() {
        let n = Number::from_i32(42);
        assert_eq!(*n.as_int(), Int::from_i32(42));
    }

    #[test]
    fn as_uint() {
        let n = Number::from_u32(42);
        assert_eq!(*n.as_uint(), UInt::from_u32(42));
    }

    #[test]
    #[should_panic(expected = "expected Float")]
    fn as_float_panics_on_mismatch() {
        Number::from_i32(1).as_float();
    }

    #[test]
    #[should_panic(expected = "expected Int")]
    fn as_int_panics_on_mismatch() {
        Number::from_f64(1.0).as_int();
    }

    #[test]
    #[should_panic(expected = "expected UInt")]
    fn as_uint_panics_on_mismatch() {
        Number::from_i32(1).as_uint();
    }

    #[test]
    fn into_value() {
        let v = Value::from_i32(5);
        assert!(matches!(v, Value::Number(_)));
    }

    #[test]
    fn display() {
        assert_eq!(Number::from_f64(1.5).to_string(), "1.5");
        assert_eq!(Number::from_i32(42).to_string(), "42");
        assert_eq!(Number::from_u64(100).to_string(), "100");
    }

    #[test]
    fn type_id() {
        assert_eq!(
            Number::from_f32(1.0).type_id(),
            std::any::TypeId::of::<f32>()
        );
        assert_eq!(
            Number::from_f64(1.0).type_id(),
            std::any::TypeId::of::<f64>()
        );
        assert_eq!(Number::from_i32(1).type_id(), std::any::TypeId::of::<i32>());
        assert_eq!(Number::from_u32(1).type_id(), std::any::TypeId::of::<u32>());
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            assert_eq!(
                serde_json::to_string(&Number::from_f64(3.14)).unwrap(),
                "3.14"
            );
            assert_eq!(serde_json::to_string(&Number::from_i32(42)).unwrap(), "42");
            assert_eq!(serde_json::to_string(&Number::from_i32(-5)).unwrap(), "-5");
        }

        #[test]
        fn deserialize() {
            let n: Number = serde_json::from_str("3.14").unwrap();
            assert!(n.is_float());

            let n: Number = serde_json::from_str("42").unwrap();
            assert!(n.is_int());
        }
    }
}
