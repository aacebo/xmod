use crate::{ToValue, Value};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Bool(bool);

impl Bool {
    pub fn to_bool(&self) -> bool {
        self.0
    }
}

impl Bool {
    pub fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<bool>()
    }
}

impl From<Bool> for Value {
    fn from(value: Bool) -> Self {
        Self::Bool(value)
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Bool::from(value).into()
    }
}

impl std::ops::Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl ToValue for Bool {
    fn to_value(self) -> Value {
        Value::from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bool() {
        assert!(Bool::from(true).to_bool());
        assert!(!Bool::from(false).to_bool());
    }

    #[test]
    fn from_bool() {
        let b = Bool::from(true);
        assert_eq!(b, Bool(true));

        let b = Bool::from(false);
        assert_eq!(b, Bool(false));
    }

    #[test]
    fn into_value() {
        let v = Value::from(true);
        assert!(matches!(v, Value::Bool(_)));
    }

    #[test]
    fn deref() {
        let b = Bool::from(true);
        assert_eq!(*b, true);

        let b = Bool::from(false);
        assert_eq!(*b, false);
    }

    #[test]
    fn display() {
        assert_eq!(Bool::from(true).to_string(), "true");
        assert_eq!(Bool::from(false).to_string(), "false");
    }

    #[test]
    fn type_id() {
        assert_eq!(Bool::from(true).type_id(), std::any::TypeId::of::<bool>());
    }
}
