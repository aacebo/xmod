mod bool;
pub mod num;

pub use bool::*;
pub use num::*;

pub trait ToValue {
    fn to_value(self) -> Value;
}

pub trait AsValue {
    fn as_value<'a>(&self) -> &'a Value;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(Bool),
    Number(Number),
}

impl Value {
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn as_bool(&self) -> &Bool {
        match self {
            Self::Bool(v) => v,
            v => panic!(
                "{}",
                format!("expected Bool, received {}", std::any::type_name_of_val(v))
            ),
        }
    }

    pub fn as_number(&self) -> &Number {
        match self {
            Self::Number(v) => v,
            v => panic!(
                "{}",
                format!(
                    "expected Number, received {}",
                    std::any::type_name_of_val(v)
                )
            ),
        }
    }
}
