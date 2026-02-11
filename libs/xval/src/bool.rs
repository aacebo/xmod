use crate::{ToValue, Value};

/// A type-safe wrapper around a [`bool`] value.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Bool(bool);

impl Bool {
    pub fn from_bool(value: bool) -> Self {
        Self(value)
    }

    pub fn to_bool(&self) -> bool {
        self.0
    }

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
        Self::from_bool(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl Value {
    pub fn from_bool(value: bool) -> Self {
        Self::Bool(Bool::from_bool(value))
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
        assert!(Bool::from_bool(true).to_bool());
        assert!(!Bool::from_bool(false).to_bool());
    }

    #[test]
    fn from_bool() {
        let b = Bool::from_bool(true);
        assert_eq!(b, Bool(true));

        let b = Bool::from_bool(false);
        assert_eq!(b, Bool(false));
    }

    #[test]
    fn into_value() {
        let v = Value::from_bool(true);
        assert!(matches!(v, Value::Bool(_)));
    }

    #[test]
    fn deref() {
        let b = Bool::from_bool(true);
        assert_eq!(*b, true);

        let b = Bool::from_bool(false);
        assert_eq!(*b, false);
    }

    #[test]
    fn display() {
        assert_eq!(Bool::from_bool(true).to_string(), "true");
        assert_eq!(Bool::from_bool(false).to_string(), "false");
    }

    #[test]
    fn type_id() {
        assert_eq!(
            Bool::from_bool(true).type_id(),
            std::any::TypeId::of::<bool>()
        );
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            assert_eq!(
                serde_json::to_string(&Bool::from_bool(true)).unwrap(),
                "true"
            );
            assert_eq!(
                serde_json::to_string(&Bool::from_bool(false)).unwrap(),
                "false"
            );
        }

        #[test]
        fn deserialize() {
            let b: Bool = serde_json::from_str("true").unwrap();
            assert_eq!(b, Bool::from_bool(true));

            let b: Bool = serde_json::from_str("false").unwrap();
            assert_eq!(b, Bool::from_bool(false));
        }
    }
}
