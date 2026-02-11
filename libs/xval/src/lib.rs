mod bool;
pub mod num;

pub use bool::*;
pub use num::*;

pub trait ToValue {
    fn to_value(self) -> Value;
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

impl Value {
    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::Bool(v) => v.type_id(),
            Self::Number(v) => v.type_id(),
        }
    }
}

impl Value {
    pub fn to_bool(&self) -> bool {
        self.as_bool().to_bool()
    }
}

impl Value {
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_float())
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_int())
    }

    pub fn is_uint(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_uint())
    }

    pub fn as_float(&self) -> &Float {
        self.as_number().as_float()
    }

    pub fn as_int(&self) -> &Int {
        self.as_number().as_int()
    }

    pub fn as_uint(&self) -> &UInt {
        self.as_number().as_uint()
    }
}

impl Value {
    pub fn is_f32(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_f32())
    }

    pub fn is_f64(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_f64())
    }

    pub fn to_f32(&self) -> f32 {
        self.as_number().to_f32()
    }

    pub fn to_f64(&self) -> f64 {
        self.as_number().to_f64()
    }
}

impl Value {
    pub fn is_i8(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_i8())
    }

    pub fn is_i16(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_i16())
    }

    pub fn is_i32(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_i32())
    }

    pub fn is_i64(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_i64())
    }

    pub fn to_i8(&self) -> i8 {
        self.as_number().to_i8()
    }

    pub fn to_i16(&self) -> i16 {
        self.as_number().to_i16()
    }

    pub fn to_i32(&self) -> i32 {
        self.as_number().to_i32()
    }

    pub fn to_i64(&self) -> i64 {
        self.as_number().to_i64()
    }
}

impl Value {
    pub fn is_u8(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_u8())
    }

    pub fn is_u16(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_u16())
    }

    pub fn is_u32(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_u32())
    }

    pub fn is_u64(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_u64())
    }

    pub fn to_u8(&self) -> u8 {
        self.as_number().to_u8()
    }

    pub fn to_u16(&self) -> u16 {
        self.as_number().to_u16()
    }

    pub fn to_u32(&self) -> u32 {
        self.as_number().to_u32()
    }

    pub fn to_u64(&self) -> u64 {
        self.as_number().to_u64()
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
        let b = Value::from(true);
        assert!(b.is_bool());
        assert!(!b.is_number());

        let n = Value::from(1i32);
        assert!(n.is_number());
        assert!(!n.is_bool());
    }

    #[test]
    fn as_bool() {
        let v = Value::from(true);
        assert_eq!(v.to_bool(), true);
    }

    #[test]
    fn as_number() {
        let v = Value::from(42i32);
        assert_eq!(*v.as_number(), Number::from(42i32));
    }

    #[test]
    #[should_panic(expected = "expected Bool")]
    fn as_bool_panics_on_mismatch() {
        Value::from(1i32).as_bool();
    }

    #[test]
    #[should_panic(expected = "expected Number")]
    fn as_number_panics_on_mismatch() {
        Value::from(true).as_number();
    }

    #[test]
    fn display() {
        assert_eq!(Value::from(true).to_string(), "true");
        assert_eq!(Value::from(42i32).to_string(), "42");
        assert_eq!(Value::from(3.14f64).to_string(), "3.14");
    }

    #[test]
    fn type_id() {
        assert_eq!(Value::from(true).type_id(), std::any::TypeId::of::<bool>());
        assert_eq!(Value::from(1.0f32).type_id(), std::any::TypeId::of::<f32>());
        assert_eq!(Value::from(1.0f64).type_id(), std::any::TypeId::of::<f64>());
        assert_eq!(Value::from(1i32).type_id(), std::any::TypeId::of::<i32>());
        assert_eq!(Value::from(1u32).type_id(), std::any::TypeId::of::<u32>());
    }
}
