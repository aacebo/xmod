#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum Segment {
    Key(String),
    Index(usize),
}

impl Segment {
    pub fn parse(src: &str) -> Self {
        if let Ok(index) = src.parse::<usize>() {
            return Self::Index(index);
        }

        Self::Key(src.to_string())
    }
}

impl From<&str> for Segment {
    fn from(value: &str) -> Self {
        Self::parse(value)
    }
}

impl From<String> for Segment {
    fn from(value: String) -> Self {
        Self::parse(&value)
    }
}

impl From<usize> for Segment {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(v) => write!(f, "{}", v),
            Self::Index(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    mod serde_tests {
        use crate::Segment;

        #[test]
        fn serialize_key() {
            let json = serde_json::to_string(&Segment::Key("name".into())).unwrap();
            assert_eq!(json, r#""name""#);
        }

        #[test]
        fn serialize_index() {
            let json = serde_json::to_string(&Segment::Index(42)).unwrap();
            assert_eq!(json, "42");
        }

        #[test]
        fn deserialize_key() {
            let seg: Segment = serde_json::from_str(r#""name""#).unwrap();
            assert_eq!(seg, Segment::Key("name".into()));
        }

        #[test]
        fn deserialize_index() {
            let seg: Segment = serde_json::from_str("42").unwrap();
            assert_eq!(seg, Segment::Index(42));
        }

        #[test]
        fn roundtrip_key() {
            let original = Segment::Key("users".into());
            let json = serde_json::to_string(&original).unwrap();
            let restored: Segment = serde_json::from_str(&json).unwrap();
            assert_eq!(original, restored);
        }

        #[test]
        fn roundtrip_index() {
            let original = Segment::Index(7);
            let json = serde_json::to_string(&original).unwrap();
            let restored: Segment = serde_json::from_str(&json).unwrap();
            assert_eq!(original, restored);
        }
    }
}
