use crate::num::Number;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum Float {
    F32(f32),
    F64(f64),
}

impl Float {
    pub fn is_f32(&self) -> bool {
        matches!(self, Self::F32(_))
    }

    pub fn is_f64(&self) -> bool {
        matches!(self, Self::F64(_))
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            Self::F32(v) => *v,
            v => panic!("expected f32, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Self::F64(v) => *v,
            v => panic!("expected f64, received {}", std::any::type_name_of_val(v)),
        }
    }
}

impl From<Float> for Number {
    fn from(value: Float) -> Self {
        Self::Float(value)
    }
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F32(v) => write!(f, "{}", v),
            Self::F64(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let v = Float::F32(1.0);
        assert!(v.is_f32());
        assert!(!v.is_f64());

        assert!(Float::F64(1.0).is_f64());
    }

    #[test]
    fn to_f32() {
        assert_eq!(Float::F32(3.14).to_f32(), 3.14);
    }

    #[test]
    fn to_f64() {
        assert_eq!(Float::F64(3.14).to_f64(), 3.14);
    }

    #[test]
    #[should_panic(expected = "expected f32")]
    fn to_f32_panics_on_mismatch() {
        Float::F64(1.0).to_f32();
    }

    #[test]
    #[should_panic(expected = "expected f64")]
    fn to_f64_panics_on_mismatch() {
        Float::F32(1.0).to_f64();
    }

    #[test]
    fn from_primitives() {
        assert_eq!(Float::from(1.0f32), Float::F32(1.0));
        assert_eq!(Float::from(1.0f64), Float::F64(1.0));
    }

    #[test]
    fn into_number() {
        let n = Number::from(Float::F64(2.5));
        assert!(matches!(n, Number::Float(Float::F64(v)) if v == 2.5));
    }

    #[test]
    fn display() {
        assert_eq!(Float::F32(1.5).to_string(), "1.5");
        assert_eq!(Float::F64(2.5).to_string(), "2.5");
    }
}
