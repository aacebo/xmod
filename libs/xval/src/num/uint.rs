use crate::{AsValue, Value, num::Number};

/// An unsigned integer value that can hold a [`u8`], [`u16`], [`u32`], [`u64`], or [`u128`].
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
    U128(u128),
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

    pub fn from_u128(value: u128) -> Self {
        Self::U128(value)
    }

    pub fn from_usize(value: usize) -> Self {
        if value <= u8::MAX as usize {
            Self::U8(value as u8)
        } else if value <= u16::MAX as usize {
            Self::U16(value as u16)
        } else if value <= u32::MAX as usize {
            Self::U32(value as u32)
        } else {
            Self::U64(value as u64)
        }
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

    pub fn is_u128(&self) -> bool {
        matches!(self, Self::U128(_))
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::U8(v) => *v,
            Self::U16(v) => *v as u8,
            Self::U32(v) => *v as u8,
            Self::U64(v) => *v as u8,
            Self::U128(v) => *v as u8,
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Self::U8(v) => *v as u16,
            Self::U16(v) => *v,
            Self::U32(v) => *v as u16,
            Self::U64(v) => *v as u16,
            Self::U128(v) => *v as u16,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::U8(v) => *v as u32,
            Self::U16(v) => *v as u32,
            Self::U32(v) => *v,
            Self::U64(v) => *v as u32,
            Self::U128(v) => *v as u32,
        }
    }

    pub fn to_u64(&self) -> u64 {
        match self {
            Self::U8(v) => *v as u64,
            Self::U16(v) => *v as u64,
            Self::U32(v) => *v as u64,
            Self::U64(v) => *v,
            Self::U128(v) => *v as u64,
        }
    }

    pub fn to_u128(&self) -> u128 {
        match self {
            Self::U8(v) => *v as u128,
            Self::U16(v) => *v as u128,
            Self::U32(v) => *v as u128,
            Self::U64(v) => *v as u128,
            Self::U128(v) => *v,
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::U8(v) => *v as usize,
            Self::U16(v) => *v as usize,
            Self::U32(v) => *v as usize,
            Self::U64(v) => *v as usize,
            Self::U128(v) => *v as usize,
        }
    }

    pub fn to_i8(&self) -> i8 {
        match self {
            Self::U8(v) => *v as i8,
            Self::U16(v) => *v as i8,
            Self::U32(v) => *v as i8,
            Self::U64(v) => *v as i8,
            Self::U128(v) => *v as i8,
        }
    }

    pub fn to_i16(&self) -> i16 {
        match self {
            Self::U8(v) => *v as i16,
            Self::U16(v) => *v as i16,
            Self::U32(v) => *v as i16,
            Self::U64(v) => *v as i16,
            Self::U128(v) => *v as i16,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::U8(v) => *v as i32,
            Self::U16(v) => *v as i32,
            Self::U32(v) => *v as i32,
            Self::U64(v) => *v as i32,
            Self::U128(v) => *v as i32,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Self::U8(v) => *v as i64,
            Self::U16(v) => *v as i64,
            Self::U32(v) => *v as i64,
            Self::U64(v) => *v as i64,
            Self::U128(v) => *v as i64,
        }
    }

    pub fn to_i128(&self) -> i128 {
        match self {
            Self::U8(v) => *v as i128,
            Self::U16(v) => *v as i128,
            Self::U32(v) => *v as i128,
            Self::U64(v) => *v as i128,
            Self::U128(v) => *v as i128,
        }
    }

    pub fn to_isize(&self) -> isize {
        match self {
            Self::U8(v) => *v as isize,
            Self::U16(v) => *v as isize,
            Self::U32(v) => *v as isize,
            Self::U64(v) => *v as isize,
            Self::U128(v) => *v as isize,
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            Self::U8(v) => *v as f32,
            Self::U16(v) => *v as f32,
            Self::U32(v) => *v as f32,
            Self::U64(v) => *v as f32,
            Self::U128(v) => *v as f32,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Self::U8(v) => *v as f64,
            Self::U16(v) => *v as f64,
            Self::U32(v) => *v as f64,
            Self::U64(v) => *v as f64,
            Self::U128(v) => *v as f64,
        }
    }

    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::U8(_) => std::any::TypeId::of::<u8>(),
            Self::U16(_) => std::any::TypeId::of::<u16>(),
            Self::U32(_) => std::any::TypeId::of::<u32>(),
            Self::U64(_) => std::any::TypeId::of::<u64>(),
            Self::U128(_) => std::any::TypeId::of::<u128>(),
        }
    }
}

impl PartialOrd for UInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_u128().cmp(&other.to_u128())
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

impl PartialEq<u128> for UInt {
    fn eq(&self, other: &u128) -> bool {
        matches!(self, Self::U128(v) if v == other)
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

impl PartialEq<u128> for Number {
    fn eq(&self, other: &u128) -> bool {
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

impl PartialEq<u128> for Value {
    fn eq(&self, other: &u128) -> bool {
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

impl From<u128> for UInt {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<usize> for UInt {
    fn from(value: usize) -> Self {
        Self::from_usize(value)
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

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Self::from_usize(value)
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

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self::from_usize(value)
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

    pub fn from_u128(value: u128) -> Self {
        Self::UInt(UInt::from_u128(value))
    }

    pub fn from_usize(value: usize) -> Self {
        Self::UInt(UInt::from_usize(value))
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

    pub fn from_u128(value: u128) -> Self {
        Self::Number(Number::from_u128(value))
    }

    pub fn from_usize(value: usize) -> Self {
        Self::Number(Number::from_usize(value))
    }
}

impl std::fmt::Debug for UInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{:#?}", v),
            Self::U16(v) => write!(f, "{:#?}", v),
            Self::U32(v) => write!(f, "{:#?}", v),
            Self::U64(v) => write!(f, "{:#?}", v),
            Self::U128(v) => write!(f, "{:#?}", v),
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
            Self::U128(v) => write!(f, "{}", v),
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

impl AsValue for u128 {
    fn as_value(&self) -> Value {
        Value::from_u128(*self)
    }
}

impl AsValue for usize {
    fn as_value(&self) -> Value {
        Value::from_usize(*self)
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
        assert!(!v.is_u128());

        assert!(UInt::from_u16(1).is_u16());
        assert!(UInt::from_u32(1).is_u32());
        assert!(UInt::from_u64(1).is_u64());
        assert!(UInt::from_u128(1).is_u128());
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
    fn to_u128() {
        assert_eq!(UInt::from_u128(42).to_u128(), 42);
    }

    #[test]
    fn to_u64_cross_variant() {
        assert_eq!(UInt::from_u8(5).to_u64(), 5);
        assert_eq!(UInt::from_u16(300).to_u64(), 300);
        assert_eq!(UInt::from_u32(100_000).to_u64(), 100_000);
    }

    #[test]
    fn to_u128_cross_variant() {
        assert_eq!(UInt::from_u8(5).to_u128(), 5);
        assert_eq!(UInt::from_u64(100_000).to_u128(), 100_000);
    }

    #[test]
    fn to_u8_truncates() {
        assert_eq!(UInt::from_u64(500).to_u8(), 500u64 as u8);
    }

    #[test]
    fn to_i64_from_uint() {
        assert_eq!(UInt::from_u32(42).to_i64(), 42);
    }

    #[test]
    fn to_f64_from_uint() {
        assert_eq!(UInt::from_u32(42).to_f64(), 42.0);
    }

    #[test]
    fn to_usize() {
        assert_eq!(UInt::from_u32(42).to_usize(), 42);
    }

    #[test]
    fn to_isize() {
        assert_eq!(UInt::from_u32(42).to_isize(), 42);
    }

    #[test]
    fn from_primitives() {
        assert_eq!(UInt::from_u8(1), UInt::from_u8(1));
        assert_eq!(UInt::from_u16(1), UInt::from_u16(1));
        assert_eq!(UInt::from_u32(1), UInt::from_u32(1));
        assert_eq!(UInt::from_u64(1), UInt::from_u64(1));
        assert_eq!(UInt::from_u128(1), UInt::from_u128(1));
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
        assert_eq!(
            UInt::from_u128(340282366920938463463374607431768211455u128).to_string(),
            "340282366920938463463374607431768211455"
        );
    }

    #[test]
    fn type_id() {
        assert_eq!(UInt::from_u8(1).type_id(), std::any::TypeId::of::<u8>());
        assert_eq!(UInt::from_u16(1).type_id(), std::any::TypeId::of::<u16>());
        assert_eq!(UInt::from_u32(1).type_id(), std::any::TypeId::of::<u32>());
        assert_eq!(UInt::from_u64(1).type_id(), std::any::TypeId::of::<u64>());
        assert_eq!(UInt::from_u128(1).type_id(), std::any::TypeId::of::<u128>());
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
