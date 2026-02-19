use crate::{ToValue, Value, num::Number};

/// A floating-point value that can hold an [`f32`] or [`f64`].
#[derive(Copy, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum Float {
    F64(f64),
    F32(f32),
}

impl Float {
    pub fn from_f64(value: f64) -> Self {
        Self::F64(value)
    }

    pub fn from_f32(value: f32) -> Self {
        Self::F32(value)
    }

    pub fn is_f64(&self) -> bool {
        matches!(self, Self::F64(_))
    }

    pub fn is_f32(&self) -> bool {
        matches!(self, Self::F32(_))
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Self::F64(v) => *v,
            Self::F32(v) => *v as f64,
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            Self::F32(v) => *v,
            Self::F64(v) => *v as f32,
        }
    }

    pub fn to_i8(&self) -> i8 {
        match self {
            Self::F32(v) => *v as i8,
            Self::F64(v) => *v as i8,
        }
    }

    pub fn to_i16(&self) -> i16 {
        match self {
            Self::F32(v) => *v as i16,
            Self::F64(v) => *v as i16,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::F32(v) => *v as i32,
            Self::F64(v) => *v as i32,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Self::F32(v) => *v as i64,
            Self::F64(v) => *v as i64,
        }
    }

    pub fn to_i128(&self) -> i128 {
        match self {
            Self::F32(v) => *v as i128,
            Self::F64(v) => *v as i128,
        }
    }

    pub fn to_isize(&self) -> isize {
        match self {
            Self::F32(v) => *v as isize,
            Self::F64(v) => *v as isize,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::F32(v) => *v as u8,
            Self::F64(v) => *v as u8,
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Self::F32(v) => *v as u16,
            Self::F64(v) => *v as u16,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::F32(v) => *v as u32,
            Self::F64(v) => *v as u32,
        }
    }

    pub fn to_u64(&self) -> u64 {
        match self {
            Self::F32(v) => *v as u64,
            Self::F64(v) => *v as u64,
        }
    }

    pub fn to_u128(&self) -> u128 {
        match self {
            Self::F32(v) => *v as u128,
            Self::F64(v) => *v as u128,
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::F32(v) => *v as usize,
            Self::F64(v) => *v as usize,
        }
    }

    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::F64(_) => std::any::TypeId::of::<f64>(),
            Self::F32(_) => std::any::TypeId::of::<f32>(),
        }
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Float {}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::F32(a), Self::F32(b)) => a.total_cmp(b),
            (Self::F64(a), Self::F64(b)) => a.total_cmp(b),
            _ => self.to_f64().total_cmp(&other.to_f64()),
        }
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<f32> for Float {
    fn eq(&self, other: &f32) -> bool {
        self.to_f64().total_cmp(&(*other as f64)) == std::cmp::Ordering::Equal
    }
}

impl PartialEq<f64> for Float {
    fn eq(&self, other: &f64) -> bool {
        self.to_f64().total_cmp(other) == std::cmp::Ordering::Equal
    }
}

impl PartialEq<f32> for Number {
    fn eq(&self, other: &f32) -> bool {
        matches!(self, Self::Float(v) if v == other)
    }
}

impl PartialEq<f64> for Number {
    fn eq(&self, other: &f64) -> bool {
        matches!(self, Self::Float(v) if v == other)
    }
}

impl PartialEq<f32> for Value {
    fn eq(&self, other: &f32) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        matches!(self, Self::Number(v) if v == other)
    }
}

impl From<Float> for Number {
    fn from(value: Float) -> Self {
        Self::Float(value)
    }
}

impl From<Float> for Value {
    fn from(value: Float) -> Self {
        Number::from(value).into()
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
    }
}

impl Number {
    pub fn from_f32(value: f32) -> Self {
        Self::Float(Float::from_f32(value))
    }

    pub fn from_f64(value: f64) -> Self {
        Self::Float(Float::from_f64(value))
    }
}

impl Value {
    pub fn from_f32(value: f32) -> Self {
        Self::Number(Number::from_f32(value))
    }

    pub fn from_f64(value: f64) -> Self {
        Self::Number(Number::from_f64(value))
    }
}

impl std::fmt::Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F64(v) => write!(f, "{:#?}", v),
            Self::F32(v) => write!(f, "{:#?}", v),
        }
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F64(v) => write!(f, "{}", v),
            Self::F32(v) => write!(f, "{}", v),
        }
    }
}

impl ToValue for Float {
    fn to_value(&self) -> Value {
        Value::Number(Number::Float(*self))
    }
}

impl ToValue for f32 {
    fn to_value(&self) -> Value {
        Value::from_f32(*self)
    }
}

impl ToValue for f64 {
    fn to_value(&self) -> Value {
        Value::from_f64(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let v = Float::from_f32(1.0);
        assert!(v.is_f32());
        assert!(!v.is_f64());

        assert!(Float::from_f64(1.0).is_f64());
    }

    #[test]
    fn to_f32() {
        assert_eq!(Float::from_f32(3.14).to_f32(), 3.14);
    }

    #[test]
    fn to_f64() {
        assert_eq!(Float::from_f64(3.14).to_f64(), 3.14);
    }

    #[test]
    fn to_f64_cross_variant() {
        let v = Float::from_f32(1.5).to_f64();
        assert!((v - 1.5).abs() < 0.001);
    }

    #[test]
    fn to_f32_cross_variant() {
        assert_eq!(Float::from_f64(1.5).to_f32(), 1.5);
    }

    #[test]
    fn to_i64_from_float() {
        assert_eq!(Float::from_f64(3.14).to_i64(), 3);
        assert_eq!(Float::from_f32(2.9).to_i64(), 2);
    }

    #[test]
    fn to_i128_from_float() {
        assert_eq!(Float::from_f64(42.7).to_i128(), 42);
    }

    #[test]
    fn to_u64_from_float() {
        assert_eq!(Float::from_f64(42.7).to_u64(), 42);
    }

    #[test]
    fn to_u128_from_float() {
        assert_eq!(Float::from_f64(42.7).to_u128(), 42);
    }

    #[test]
    fn to_isize_from_float() {
        assert_eq!(Float::from_f64(42.7).to_isize(), 42);
    }

    #[test]
    fn to_usize_from_float() {
        assert_eq!(Float::from_f64(42.7).to_usize(), 42);
    }

    #[test]
    fn from_primitives() {
        assert_eq!(Float::from_f32(1.0), Float::from_f32(1.0));
        assert_eq!(Float::from_f64(1.0), Float::from_f64(1.0));
    }

    #[test]
    fn into_number() {
        let n = Number::from_f64(2.5);
        assert!(n.is_float());
        assert_eq!(n.as_float().to_f64(), 2.5);
    }

    #[test]
    fn display() {
        assert_eq!(Float::from_f32(1.5).to_string(), "1.5");
        assert_eq!(Float::from_f64(2.5).to_string(), "2.5");
    }

    #[test]
    fn type_id() {
        assert_eq!(
            Float::from_f32(1.0).type_id(),
            std::any::TypeId::of::<f32>()
        );
        assert_eq!(
            Float::from_f64(1.0).type_id(),
            std::any::TypeId::of::<f64>()
        );
    }

    #[test]
    fn nan_eq_nan() {
        assert_eq!(Float::F64(f64::NAN), Float::F64(f64::NAN));
        assert_eq!(Float::F32(f32::NAN), Float::F32(f32::NAN));
    }

    #[test]
    fn nan_value_eq() {
        let a = Value::Number(Number::Float(Float::F64(f64::NAN)));
        let b = Value::Number(Number::Float(Float::F64(f64::NAN)));
        assert_eq!(a, b);
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            assert_eq!(serde_json::to_string(&Float::from_f32(1.5)).unwrap(), "1.5");
            assert_eq!(
                serde_json::to_string(&Float::from_f64(3.14)).unwrap(),
                "3.14"
            );
        }

        #[test]
        fn deserialize() {
            let f: Float = serde_json::from_str("3.14").unwrap();
            assert!(f.is_f64());
        }
    }
}
