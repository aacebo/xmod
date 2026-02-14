use crate::{AsValue, Value};

/// A type-safe wrapper around a [`str`] value.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        Self::String(value)
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
        Self::String(Str::from_str(value))
    }

    pub fn from_string(value: String) -> Self {
        Self::String(Str::from_string(value))
    }
}

impl std::ops::Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self.0)
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl PartialEq<str> for Str {
    fn eq(&self, other: &str) -> bool {
        &*self.0 == other
    }
}

impl PartialEq<&str> for Str {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for Str {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        matches!(self, Self::String(v) if v == other)
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl AsValue for Str {
    fn as_value(&self) -> Value {
        Value::String(self.clone())
    }
}

impl AsValue for str {
    fn as_value(&self) -> Value {
        Value::from_str(self)
    }
}

impl AsValue for String {
    fn as_value(&self) -> Value {
        Value::from_string(self.clone())
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
        assert!(matches!(v, Value::String(_)));
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
