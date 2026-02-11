use crate::{ToValue, Value};

/// A type-safe wrapper around a [`str`] value.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
pub struct Str(Box<str>);

impl Str {
    pub fn from_str(value: &str) -> Self {
        Self(value.into())
    }

    pub fn from_string(value: String) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<String>()
    }
}

impl From<Str> for Value {
    fn from(value: Str) -> Self {
        Self::Str(value)
    }
}

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<String> for Str {
    fn from(value: String) -> Self {
        Self::from_string(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::from_string(value)
    }
}

impl Value {
    pub fn from_str(value: &str) -> Self {
        Self::Str(Str::from_str(value))
    }

    pub fn from_string(value: String) -> Self {
        Self::Str(Str::from_string(value))
    }
}

impl std::ops::Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl ToValue for Str {
    fn to_value(self) -> Value {
        Value::from(self)
    }
}

impl ToValue for &str {
    fn to_value(self) -> Value {
        Value::from_str(self)
    }
}

impl ToValue for String {
    fn to_value(self) -> Value {
        Value::from_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let s = Str::from_str("hello");
        assert_eq!(s, Str::from_str("hello"));
    }

    #[test]
    fn from_string() {
        let s = Str::from_string(String::from("hello"));
        assert_eq!(s, Str::from_str("hello"));
    }

    #[test]
    fn as_str() {
        let s = Str::from_str("hello");
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn into_value() {
        let v = Value::from_str("hello");
        assert!(matches!(v, Value::Str(_)));
    }

    #[test]
    fn deref() {
        let s = Str::from_str("hello");
        assert_eq!(&*s, "hello");
    }

    #[test]
    fn display() {
        assert_eq!(Str::from_str("hello").to_string(), "hello");
        assert_eq!(Str::from_str("").to_string(), "");
    }

    #[test]
    fn type_id() {
        assert_eq!(
            Str::from_str("hello").type_id(),
            std::any::TypeId::of::<String>()
        );
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            assert_eq!(
                serde_json::to_string(&Str::from_str("hello")).unwrap(),
                "\"hello\""
            );
            assert_eq!(serde_json::to_string(&Str::from_str("")).unwrap(), "\"\"");
        }

        #[test]
        fn deserialize() {
            let s: Str = serde_json::from_str("\"hello\"").unwrap();
            assert_eq!(s, Str::from_str("hello"));

            let s: Str = serde_json::from_str("\"\"").unwrap();
            assert_eq!(s, Str::from_str(""));
        }
    }
}
