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
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
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
            v => panic!("expected Bool, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn as_number(&self) -> &Number {
        match self {
            Self::Number(v) => v,
            v => panic!(
                "expected Number, received {}",
                std::any::type_name_of_val(v)
            ),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let b = Value::Bool(Bool::from(true));
        assert!(b.is_bool());
        assert!(!b.is_number());

        let n = Value::Number(Number::Int(Int::I32(1)));
        assert!(n.is_number());
        assert!(!n.is_bool());
    }

    #[test]
    fn as_bool() {
        let v = Value::Bool(Bool::from(true));
        assert_eq!(v.as_bool().to_bool(), true);
    }

    #[test]
    fn as_number() {
        let v = Value::Number(Number::Int(Int::I32(42)));
        assert_eq!(*v.as_number(), Number::Int(Int::I32(42)));
    }

    #[test]
    #[should_panic(expected = "expected Bool")]
    fn as_bool_panics_on_mismatch() {
        Value::Number(Number::Int(Int::I32(1))).as_bool();
    }

    #[test]
    #[should_panic(expected = "expected Number")]
    fn as_number_panics_on_mismatch() {
        Value::Bool(Bool::from(true)).as_number();
    }

    #[test]
    fn display() {
        assert_eq!(Value::Bool(Bool::from(true)).to_string(), "true");
        assert_eq!(Value::Number(Number::Int(Int::I32(42))).to_string(), "42");
        assert_eq!(
            Value::Number(Number::Float(Float::F64(3.14))).to_string(),
            "3.14"
        );
    }
}
