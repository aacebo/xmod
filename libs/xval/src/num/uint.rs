use crate::{AsValue, Value, num::Number};

/// An unsigned integer value that can hold a [`u8`], [`u16`], [`u32`], or [`u64`].
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum UInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl UInt {
    pub fn from_u8(value: u8) -> Self {
        Self::U8(value)
    }
    pub fn from_u16(value: u16) -> Self {
        Self::U16(value)
    }
    pub fn from_u32(value: u32) -> Self {
        Self::U32(value)
    }
    pub fn from_u64(value: u64) -> Self {
        Self::U64(value)
    }

    pub fn is_u8(&self) -> bool {
        matches!(self, Self::U8(_))
    }
    pub fn is_u16(&self) -> bool {
        matches!(self, Self::U16(_))
    }
    pub fn is_u32(&self) -> bool {
        matches!(self, Self::U32(_))
    }
    pub fn is_u64(&self) -> bool {
        matches!(self, Self::U64(_))
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::U8(v) => *v,
            v => panic!("expected u8, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Self::U16(v) => *v,
            v => panic!("expected u16, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::U32(v) => *v,
            v => panic!("expected u32, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn to_u64(&self) -> u64 {
        match self {
            Self::U64(v) => *v,
            v => panic!("expected u64, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::U8(_) => std::any::TypeId::of::<u8>(),
            Self::U16(_) => std::any::TypeId::of::<u16>(),
            Self::U32(_) => std::any::TypeId::of::<u32>(),
            Self::U64(_) => std::any::TypeId::of::<u64>(),
        }
    }
}

impl PartialEq<u8> for UInt {
    fn eq(&self, other: &u8) -> bool {
        matches!(self, Self::U8(v) if v == other)
    }
}
impl PartialEq<u16> for UInt {
    fn eq(&self, other: &u16) -> bool {
        matches!(self, Self::U16(v) if v == other)
    }
}
impl PartialEq<u32> for UInt {
    fn eq(&self, other: &u32) -> bool {
        matches!(self, Self::U32(v) if v == other)
    }
}
impl PartialEq<u64> for UInt {
    fn eq(&self, other: &u64) -> bool {
        matches!(self, Self::U64(v) if v == other)
    }
}

impl PartialEq<u8> for Number {
    fn eq(&self, other: &u8) -> bool {
        matches!(self, Self::UInt(v) if v == other)
    }
}
impl PartialEq<u16> for Number {
    fn eq(&self, other: &u16) -> bool {
        matches!(self, Self::UInt(v) if v == other)
    }
}
impl PartialEq<u32> for Number {
    fn eq(&self, other: &u32) -> bool {
        matches!(self, Self::UInt(v) if v == other)
    }
}
impl PartialEq<u64> for Number {
    fn eq(&self, other: &u64) -> bool {
        matches!(self, Self::UInt(v) if v == other)
    }
}

impl PartialEq<u8> for Value {
    fn eq(&self, other: &u8) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}
impl PartialEq<u16> for Value {
    fn eq(&self, other: &u16) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}
impl PartialEq<u32> for Value {
    fn eq(&self, other: &u32) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}
impl PartialEq<u64> for Value {
    fn eq(&self, other: &u64) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}

impl From<UInt> for Number {
    fn from(value: UInt) -> Self {
        Self::UInt(value)
    }
}
impl From<u8> for UInt {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}
impl From<u16> for UInt {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}
impl From<u32> for UInt {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}
impl From<u64> for UInt {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl From<UInt> for Value {
    fn from(value: UInt) -> Self {
        Number::from(value).into()
    }
}
impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}
impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}
impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}
impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}
impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}
impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}
impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl Number {
    pub fn from_u8(value: u8) -> Self {
        Self::UInt(UInt::from_u8(value))
    }
    pub fn from_u16(value: u16) -> Self {
        Self::UInt(UInt::from_u16(value))
    }
    pub fn from_u32(value: u32) -> Self {
        Self::UInt(UInt::from_u32(value))
    }
    pub fn from_u64(value: u64) -> Self {
        Self::UInt(UInt::from_u64(value))
    }
}

impl Value {
    pub fn from_u8(value: u8) -> Self {
        Self::Number(Number::from_u8(value))
    }
    pub fn from_u16(value: u16) -> Self {
        Self::Number(Number::from_u16(value))
    }
    pub fn from_u32(value: u32) -> Self {
        Self::Number(Number::from_u32(value))
    }
    pub fn from_u64(value: u64) -> Self {
        Self::Number(Number::from_u64(value))
    }
}

impl std::fmt::Debug for UInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{:#?}", v),
            Self::U16(v) => write!(f, "{:#?}", v),
            Self::U32(v) => write!(f, "{:#?}", v),
            Self::U64(v) => write!(f, "{:#?}", v),
        }
    }
}

impl std::fmt::Display for UInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{}", v),
            Self::U16(v) => write!(f, "{}", v),
            Self::U32(v) => write!(f, "{}", v),
            Self::U64(v) => write!(f, "{}", v),
        }
    }
}

impl AsValue for UInt {
    fn as_value(&self) -> Value {
        Value::Number(Number::UInt(*self))
    }
}
impl AsValue for u8 {
    fn as_value(&self) -> Value {
        Value::from_u8(*self)
    }
}
impl AsValue for u16 {
    fn as_value(&self) -> Value {
        Value::from_u16(*self)
    }
}
impl AsValue for u32 {
    fn as_value(&self) -> Value {
        Value::from_u32(*self)
    }
}
impl AsValue for u64 {
    fn as_value(&self) -> Value {
        Value::from_u64(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let v = UInt::from_u8(1);
        assert!(v.is_u8());
        assert!(!v.is_u16());
        assert!(!v.is_u32());
        assert!(!v.is_u64());

        assert!(UInt::from_u16(1).is_u16());
        assert!(UInt::from_u32(1).is_u32());
        assert!(UInt::from_u64(1).is_u64());
    }

    #[test]
    fn to_u8() {
        assert_eq!(UInt::from_u8(42).to_u8(), 42);
    }
    #[test]
    fn to_u16() {
        assert_eq!(UInt::from_u16(42).to_u16(), 42);
    }
    #[test]
    fn to_u32() {
        assert_eq!(UInt::from_u32(42).to_u32(), 42);
    }
    #[test]
    fn to_u64() {
        assert_eq!(UInt::from_u64(42).to_u64(), 42);
    }

    #[test]
    #[should_panic(expected = "expected u8")]
    fn to_u8_panics_on_mismatch() {
        UInt::from_u32(1).to_u8();
    }
    #[test]
    #[should_panic(expected = "expected u16")]
    fn to_u16_panics_on_mismatch() {
        UInt::from_u32(1).to_u16();
    }
    #[test]
    #[should_panic(expected = "expected u32")]
    fn to_u32_panics_on_mismatch() {
        UInt::from_u64(1).to_u32();
    }
    #[test]
    #[should_panic(expected = "expected u64")]
    fn to_u64_panics_on_mismatch() {
        UInt::from_u8(1).to_u64();
    }

    #[test]
    fn from_primitives() {
        assert_eq!(UInt::from_u8(1), UInt::from_u8(1));
        assert_eq!(UInt::from_u16(1), UInt::from_u16(1));
        assert_eq!(UInt::from_u32(1), UInt::from_u32(1));
        assert_eq!(UInt::from_u64(1), UInt::from_u64(1));
    }

    #[test]
    fn into_number() {
        let n = Number::from_u32(5);
        assert!(n.is_uint());
        assert_eq!(n.as_uint().to_u32(), 5);
    }

    #[test]
    fn display() {
        assert_eq!(UInt::from_u8(255).to_string(), "255");
        assert_eq!(UInt::from_u16(1000).to_string(), "1000");
        assert_eq!(UInt::from_u32(100_000).to_string(), "100000");
        assert_eq!(UInt::from_u64(999).to_string(), "999");
    }

    #[test]
    fn type_id() {
        assert_eq!(UInt::from_u8(1).type_id(), std::any::TypeId::of::<u8>());
        assert_eq!(UInt::from_u16(1).type_id(), std::any::TypeId::of::<u16>());
        assert_eq!(UInt::from_u32(1).type_id(), std::any::TypeId::of::<u32>());
        assert_eq!(UInt::from_u64(1).type_id(), std::any::TypeId::of::<u64>());
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            assert_eq!(serde_json::to_string(&UInt::from_u8(42)).unwrap(), "42");
            assert_eq!(serde_json::to_string(&UInt::from_u32(300)).unwrap(), "300");
        }

        #[test]
        fn deserialize() {
            let u: UInt = serde_json::from_str("42").unwrap();
            assert_eq!(u.to_u8(), 42);

            let u: UInt = serde_json::from_str("300").unwrap();
            assert_eq!(u.to_u16(), 300);

            let u: UInt = serde_json::from_str("70000").unwrap();
            assert_eq!(u.to_u32(), 70000);

            let u: UInt = serde_json::from_str("5000000000").unwrap();
            assert_eq!(u.to_u64(), 5000000000);
        }
    }
}
