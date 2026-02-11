use crate::num::Number;

#[derive(Debug, Copy, Clone, PartialEq)]
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
            v => panic!(
                "{}",
                format!("expected f32, received {}", std::any::type_name_of_val(v))
            ),
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Self::F64(v) => *v,
            v => panic!(
                "{}",
                format!("expected f64, received {}", std::any::type_name_of_val(v))
            ),
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
