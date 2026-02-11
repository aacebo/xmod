use crate::{ToValue, Value, num::Number};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum Int {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl Int {
    pub fn is_i8(&self) -> bool {
        matches!(self, Self::I8(_))
    }

    pub fn is_i16(&self) -> bool {
        matches!(self, Self::I16(_))
    }

    pub fn is_i32(&self) -> bool {
        matches!(self, Self::I32(_))
    }

    pub fn is_i64(&self) -> bool {
        matches!(self, Self::I64(_))
    }

    pub fn to_i8(&self) -> i8 {
        match self {
            Self::I8(v) => *v,
            v => panic!("expected i8, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn to_i16(&self) -> i16 {
        match self {
            Self::I16(v) => *v,
            v => panic!("expected i16, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::I32(v) => *v,
            v => panic!("expected i32, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Self::I64(v) => *v,
            v => panic!("expected i64, received {}", std::any::type_name_of_val(v)),
        }
    }
}

impl From<Int> for Number {
    fn from(value: Int) -> Self {
        Self::Int(value)
    }
}

impl From<i8> for Int {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl From<i16> for Int {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl From<i32> for Int {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(v) => write!(f, "{}", v),
            Self::I16(v) => write!(f, "{}", v),
            Self::I32(v) => write!(f, "{}", v),
            Self::I64(v) => write!(f, "{}", v),
        }
    }
}

impl ToValue for Int {
    fn to_value(self) -> Value {
        Number::Int(self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let v = Int::I8(1);
        assert!(v.is_i8());
        assert!(!v.is_i16());
        assert!(!v.is_i32());
        assert!(!v.is_i64());

        assert!(Int::I16(1).is_i16());
        assert!(Int::I32(1).is_i32());
        assert!(Int::I64(1).is_i64());
    }

    #[test]
    fn to_i8() {
        assert_eq!(Int::I8(42).to_i8(), 42);
    }

    #[test]
    fn to_i16() {
        assert_eq!(Int::I16(42).to_i16(), 42);
    }

    #[test]
    fn to_i32() {
        assert_eq!(Int::I32(42).to_i32(), 42);
    }

    #[test]
    fn to_i64() {
        assert_eq!(Int::I64(42).to_i64(), 42);
    }

    #[test]
    #[should_panic(expected = "expected i8")]
    fn to_i8_panics_on_mismatch() {
        Int::I32(1).to_i8();
    }

    #[test]
    #[should_panic(expected = "expected i16")]
    fn to_i16_panics_on_mismatch() {
        Int::I32(1).to_i16();
    }

    #[test]
    #[should_panic(expected = "expected i32")]
    fn to_i32_panics_on_mismatch() {
        Int::I64(1).to_i32();
    }

    #[test]
    #[should_panic(expected = "expected i64")]
    fn to_i64_panics_on_mismatch() {
        Int::I8(1).to_i64();
    }

    #[test]
    fn from_primitives() {
        assert_eq!(Int::from(1i8), Int::I8(1));
        assert_eq!(Int::from(1i16), Int::I16(1));
        assert_eq!(Int::from(1i32), Int::I32(1));
        assert_eq!(Int::from(1i64), Int::I64(1));
    }

    #[test]
    fn into_number() {
        let n = Number::from(Int::I32(5));
        assert!(matches!(n, Number::Int(Int::I32(5))));
    }

    #[test]
    fn display() {
        assert_eq!(Int::I8(-1).to_string(), "-1");
        assert_eq!(Int::I16(200).to_string(), "200");
        assert_eq!(Int::I32(100_000).to_string(), "100000");
        assert_eq!(Int::I64(-999).to_string(), "-999");
    }
}
