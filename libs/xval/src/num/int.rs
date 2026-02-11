use crate::num::Number;

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
