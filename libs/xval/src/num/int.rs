use crate::{AsValue, Value, num::Number};

/// A signed integer value that can hold an [`i8`], [`i16`], [`i32`], [`i64`], or [`i128`].
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    I128(i128),
}

impl Int {
    pub fn from_i8(value: i8) -> Self {
        Self::I8(value)
    }

    pub fn from_i16(value: i16) -> Self {
        Self::I16(value)
    }

    pub fn from_i32(value: i32) -> Self {
        Self::I32(value)
    }

    pub fn from_i64(value: i64) -> Self {
        Self::I64(value)
    }

    pub fn from_i128(value: i128) -> Self {
        Self::I128(value)
    }

    pub fn from_isize(value: isize) -> Self {
        if value >= i8::MIN as isize && value <= i8::MAX as isize {
            Self::I8(value as i8)
        } else if value >= i16::MIN as isize && value <= i16::MAX as isize {
            Self::I16(value as i16)
        } else if value >= i32::MIN as isize && value <= i32::MAX as isize {
            Self::I32(value as i32)
        } else {
            Self::I64(value as i64)
        }
    }

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

    pub fn is_i128(&self) -> bool {
        matches!(self, Self::I128(_))
    }

    pub fn to_i8(&self) -> i8 {
        match self {
            Self::I8(v) => *v,
            Self::I16(v) => *v as i8,
            Self::I32(v) => *v as i8,
            Self::I64(v) => *v as i8,
            Self::I128(v) => *v as i8,
        }
    }

    pub fn to_i16(&self) -> i16 {
        match self {
            Self::I8(v) => *v as i16,
            Self::I16(v) => *v,
            Self::I32(v) => *v as i16,
            Self::I64(v) => *v as i16,
            Self::I128(v) => *v as i16,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::I8(v) => *v as i32,
            Self::I16(v) => *v as i32,
            Self::I32(v) => *v,
            Self::I64(v) => *v as i32,
            Self::I128(v) => *v as i32,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Self::I8(v) => *v as i64,
            Self::I16(v) => *v as i64,
            Self::I32(v) => *v as i64,
            Self::I64(v) => *v,
            Self::I128(v) => *v as i64,
        }
    }

    pub fn to_i128(&self) -> i128 {
        match self {
            Self::I8(v) => *v as i128,
            Self::I16(v) => *v as i128,
            Self::I32(v) => *v as i128,
            Self::I64(v) => *v as i128,
            Self::I128(v) => *v,
        }
    }

    pub fn to_isize(&self) -> isize {
        match self {
            Self::I8(v) => *v as isize,
            Self::I16(v) => *v as isize,
            Self::I32(v) => *v as isize,
            Self::I64(v) => *v as isize,
            Self::I128(v) => *v as isize,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::I8(v) => *v as u8,
            Self::I16(v) => *v as u8,
            Self::I32(v) => *v as u8,
            Self::I64(v) => *v as u8,
            Self::I128(v) => *v as u8,
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Self::I8(v) => *v as u16,
            Self::I16(v) => *v as u16,
            Self::I32(v) => *v as u16,
            Self::I64(v) => *v as u16,
            Self::I128(v) => *v as u16,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::I8(v) => *v as u32,
            Self::I16(v) => *v as u32,
            Self::I32(v) => *v as u32,
            Self::I64(v) => *v as u32,
            Self::I128(v) => *v as u32,
        }
    }

    pub fn to_u64(&self) -> u64 {
        match self {
            Self::I8(v) => *v as u64,
            Self::I16(v) => *v as u64,
            Self::I32(v) => *v as u64,
            Self::I64(v) => *v as u64,
            Self::I128(v) => *v as u64,
        }
    }

    pub fn to_u128(&self) -> u128 {
        match self {
            Self::I8(v) => *v as u128,
            Self::I16(v) => *v as u128,
            Self::I32(v) => *v as u128,
            Self::I64(v) => *v as u128,
            Self::I128(v) => *v as u128,
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::I8(v) => *v as usize,
            Self::I16(v) => *v as usize,
            Self::I32(v) => *v as usize,
            Self::I64(v) => *v as usize,
            Self::I128(v) => *v as usize,
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            Self::I8(v) => *v as f32,
            Self::I16(v) => *v as f32,
            Self::I32(v) => *v as f32,
            Self::I64(v) => *v as f32,
            Self::I128(v) => *v as f32,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Self::I8(v) => *v as f64,
            Self::I16(v) => *v as f64,
            Self::I32(v) => *v as f64,
            Self::I64(v) => *v as f64,
            Self::I128(v) => *v as f64,
        }
    }

    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::I8(_) => std::any::TypeId::of::<i8>(),
            Self::I16(_) => std::any::TypeId::of::<i16>(),
            Self::I32(_) => std::any::TypeId::of::<i32>(),
            Self::I64(_) => std::any::TypeId::of::<i64>(),
            Self::I128(_) => std::any::TypeId::of::<i128>(),
        }
    }
}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Int {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_i128().cmp(&other.to_i128())
    }
}

impl PartialEq<i8> for Int {
    fn eq(&self, other: &i8) -> bool {
        matches!(self, Self::I8(v) if v == other)
    }
}

impl PartialEq<i16> for Int {
    fn eq(&self, other: &i16) -> bool {
        matches!(self, Self::I16(v) if v == other)
    }
}

impl PartialEq<i32> for Int {
    fn eq(&self, other: &i32) -> bool {
        matches!(self, Self::I32(v) if v == other)
    }
}

impl PartialEq<i64> for Int {
    fn eq(&self, other: &i64) -> bool {
        matches!(self, Self::I64(v) if v == other)
    }
}

impl PartialEq<i128> for Int {
    fn eq(&self, other: &i128) -> bool {
        matches!(self, Self::I128(v) if v == other)
    }
}

impl PartialEq<i8> for Number {
    fn eq(&self, other: &i8) -> bool {
        matches!(self, Self::Int(v) if v == other)
    }
}

impl PartialEq<i16> for Number {
    fn eq(&self, other: &i16) -> bool {
        matches!(self, Self::Int(v) if v == other)
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        matches!(self, Self::Int(v) if v == other)
    }
}

impl PartialEq<i64> for Number {
    fn eq(&self, other: &i64) -> bool {
        matches!(self, Self::Int(v) if v == other)
    }
}

impl PartialEq<i128> for Number {
    fn eq(&self, other: &i128) -> bool {
        matches!(self, Self::Int(v) if v == other)
    }
}

impl PartialEq<i8> for Value {
    fn eq(&self, other: &i8) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}

impl PartialEq<i16> for Value {
    fn eq(&self, other: &i16) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, other: &i32) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}

impl PartialEq<i64> for Value {
    fn eq(&self, other: &i64) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}

impl PartialEq<i128> for Value {
    fn eq(&self, other: &i128) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}

impl From<Int> for Number {
    fn from(value: Int) -> Self {
        Self::Int(value)
    }
}

impl From<i8> for Int {
    fn from(value: i8) -> Self {
        Self::from_i8(value)
    }
}

impl From<i16> for Int {
    fn from(value: i16) -> Self {
        Self::from_i16(value)
    }
}

impl From<i32> for Int {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<i128> for Int {
    fn from(value: i128) -> Self {
        Self::from_i128(value)
    }
}

impl From<isize> for Int {
    fn from(value: isize) -> Self {
        Self::from_isize(value)
    }
}

impl From<Int> for Value {
    fn from(value: Int) -> Self {
        Number::from(value).into()
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Self::from_i8(value)
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Self::from_i16(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Self::from_i128(value)
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        Self::from_isize(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::from_i8(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::from_i16(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Self::from_i128(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Self::from_isize(value)
    }
}

impl Number {
    pub fn from_i8(value: i8) -> Self {
        Self::Int(Int::from_i8(value))
    }

    pub fn from_i16(value: i16) -> Self {
        Self::Int(Int::from_i16(value))
    }

    pub fn from_i32(value: i32) -> Self {
        Self::Int(Int::from_i32(value))
    }

    pub fn from_i64(value: i64) -> Self {
        Self::Int(Int::from_i64(value))
    }

    pub fn from_i128(value: i128) -> Self {
        Self::Int(Int::from_i128(value))
    }

    pub fn from_isize(value: isize) -> Self {
        Self::Int(Int::from_isize(value))
    }
}

impl Value {
    pub fn from_i8(value: i8) -> Self {
        Self::Number(Number::from_i8(value))
    }

    pub fn from_i16(value: i16) -> Self {
        Self::Number(Number::from_i16(value))
    }

    pub fn from_i32(value: i32) -> Self {
        Self::Number(Number::from_i32(value))
    }

    pub fn from_i64(value: i64) -> Self {
        Self::Number(Number::from_i64(value))
    }

    pub fn from_i128(value: i128) -> Self {
        Self::Number(Number::from_i128(value))
    }

    pub fn from_isize(value: isize) -> Self {
        Self::Number(Number::from_isize(value))
    }
}

impl std::fmt::Debug for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(v) => write!(f, "{:#?}", v),
            Self::I16(v) => write!(f, "{:#?}", v),
            Self::I32(v) => write!(f, "{:#?}", v),
            Self::I64(v) => write!(f, "{:#?}", v),
            Self::I128(v) => write!(f, "{:#?}", v),
        }
    }
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(v) => write!(f, "{}", v),
            Self::I16(v) => write!(f, "{}", v),
            Self::I32(v) => write!(f, "{}", v),
            Self::I64(v) => write!(f, "{}", v),
            Self::I128(v) => write!(f, "{}", v),
        }
    }
}

impl AsValue for Int {
    fn as_value(&self) -> Value {
        Value::Number(Number::Int(*self))
    }
}

impl AsValue for i8 {
    fn as_value(&self) -> Value {
        Value::from_i8(*self)
    }
}

impl AsValue for i16 {
    fn as_value(&self) -> Value {
        Value::from_i16(*self)
    }
}

impl AsValue for i32 {
    fn as_value(&self) -> Value {
        Value::from_i32(*self)
    }
}

impl AsValue for i64 {
    fn as_value(&self) -> Value {
        Value::from_i64(*self)
    }
}

impl AsValue for i128 {
    fn as_value(&self) -> Value {
        Value::from_i128(*self)
    }
}

impl AsValue for isize {
    fn as_value(&self) -> Value {
        Value::from_isize(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let v = Int::from_i8(1);
        assert!(v.is_i8());
        assert!(!v.is_i16());
        assert!(!v.is_i32());
        assert!(!v.is_i64());
        assert!(!v.is_i128());

        assert!(Int::from_i16(1).is_i16());
        assert!(Int::from_i32(1).is_i32());
        assert!(Int::from_i64(1).is_i64());
        assert!(Int::from_i128(1).is_i128());
    }

    #[test]
    fn to_i8() {
        assert_eq!(Int::from_i8(42).to_i8(), 42);
    }

    #[test]
    fn to_i16() {
        assert_eq!(Int::from_i16(42).to_i16(), 42);
    }

    #[test]
    fn to_i32() {
        assert_eq!(Int::from_i32(42).to_i32(), 42);
    }

    #[test]
    fn to_i64() {
        assert_eq!(Int::from_i64(42).to_i64(), 42);
    }

    #[test]
    fn to_i128() {
        assert_eq!(Int::from_i128(42).to_i128(), 42);
    }

    #[test]
    fn to_i64_cross_variant() {
        assert_eq!(Int::from_i8(5).to_i64(), 5);
        assert_eq!(Int::from_i16(300).to_i64(), 300);
        assert_eq!(Int::from_i32(100_000).to_i64(), 100_000);
    }

    #[test]
    fn to_i128_cross_variant() {
        assert_eq!(Int::from_i8(5).to_i128(), 5);
        assert_eq!(Int::from_i64(100_000).to_i128(), 100_000);
    }

    #[test]
    fn to_i8_truncates() {
        assert_eq!(Int::from_i64(500).to_i8(), 500i64 as i8);
    }

    #[test]
    fn to_u64_from_int() {
        assert_eq!(Int::from_i32(42).to_u64(), 42);
    }

    #[test]
    fn to_f64_from_int() {
        assert_eq!(Int::from_i32(42).to_f64(), 42.0);
    }

    #[test]
    fn to_isize() {
        assert_eq!(Int::from_i32(42).to_isize(), 42);
    }

    #[test]
    fn to_usize() {
        assert_eq!(Int::from_i32(42).to_usize(), 42);
    }

    #[test]
    fn from_primitives() {
        assert_eq!(Int::from_i8(1), Int::from_i8(1));
        assert_eq!(Int::from_i16(1), Int::from_i16(1));
        assert_eq!(Int::from_i32(1), Int::from_i32(1));
        assert_eq!(Int::from_i64(1), Int::from_i64(1));
        assert_eq!(Int::from_i128(1), Int::from_i128(1));
    }

    #[test]
    fn into_number() {
        let n = Number::from_i32(5);
        assert!(n.is_int());
        assert_eq!(n.as_int().to_i32(), 5);
    }

    #[test]
    fn display() {
        assert_eq!(Int::from_i8(-1).to_string(), "-1");
        assert_eq!(Int::from_i16(200).to_string(), "200");
        assert_eq!(Int::from_i32(100_000).to_string(), "100000");
        assert_eq!(Int::from_i64(-999).to_string(), "-999");
        assert_eq!(
            Int::from_i128(170141183460469231731687303715884105727).to_string(),
            "170141183460469231731687303715884105727"
        );
    }

    #[test]
    fn type_id() {
        assert_eq!(Int::from_i8(1).type_id(), std::any::TypeId::of::<i8>());
        assert_eq!(Int::from_i16(1).type_id(), std::any::TypeId::of::<i16>());
        assert_eq!(Int::from_i32(1).type_id(), std::any::TypeId::of::<i32>());
        assert_eq!(Int::from_i64(1).type_id(), std::any::TypeId::of::<i64>());
        assert_eq!(Int::from_i128(1).type_id(), std::any::TypeId::of::<i128>());
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            assert_eq!(serde_json::to_string(&Int::from_i8(-1)).unwrap(), "-1");
            assert_eq!(serde_json::to_string(&Int::from_i32(42)).unwrap(), "42");
        }

        #[test]
        fn deserialize() {
            let i: Int = serde_json::from_str("42").unwrap();
            assert_eq!(i.to_i8(), 42);

            let i: Int = serde_json::from_str("-200").unwrap();
            assert_eq!(i.to_i16(), -200);

            let i: Int = serde_json::from_str("40000").unwrap();
            assert_eq!(i.to_i32(), 40000);

            let i: Int = serde_json::from_str("3000000000").unwrap();
            assert_eq!(i.to_i64(), 3000000000);
        }
    }
}
