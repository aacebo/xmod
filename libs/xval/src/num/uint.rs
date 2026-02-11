use crate::num::Number;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
}

impl From<UInt> for Number {
    fn from(value: UInt) -> Self {
        Self::UInt(value)
    }
}

impl From<u8> for UInt {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl From<u16> for UInt {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<u32> for UInt {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for UInt {
    fn from(value: u64) -> Self {
        Self::U64(value)
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
