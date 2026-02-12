#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum Ident {
    Key(Box<str>),
    Index(usize),
}

impl Ident {
    pub fn key(s: &str) -> Self {
        Self::Key(s.into())
    }

    pub fn index(i: usize) -> Self {
        Self::Index(i)
    }

    pub fn is_key(&self) -> bool {
        matches!(self, Self::Key(_))
    }

    pub fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }
}

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self::key(value)
    }
}

impl From<String> for Ident {
    fn from(value: String) -> Self {
        Self::Key(value.into_boxed_str())
    }
}

impl From<usize> for Ident {
    fn from(value: usize) -> Self {
        Self::index(value)
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(v) => write!(f, "{}", v),
            Self::Index(v) => write!(f, "{}", v),
        }
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.to_string() == other
    }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<String> for Ident {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl PartialEq<usize> for Ident {
    fn eq(&self, other: &usize) -> bool {
        self.to_string() == format!("{}", other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn key() {
        let k = Ident::key("foo");
        assert!(k.is_key());
        assert!(!k.is_index());
    }

    #[test]
    fn index() {
        let i = Ident::index(3);
        assert!(i.is_index());
        assert!(!i.is_key());
    }

    #[test]
    fn from_str() {
        let k: Ident = "bar".into();
        assert!(k.is_key());
        assert_eq!(k, Ident::key("bar"));
    }

    #[test]
    fn from_usize() {
        let i: Ident = 5usize.into();
        assert!(i.is_index());
        assert_eq!(i, Ident::index(5));
    }

    #[test]
    fn display_key() {
        assert_eq!(Ident::key("hello").to_string(), "hello");
    }

    #[test]
    fn display_index() {
        assert_eq!(Ident::index(42).to_string(), "42");
    }

    #[test]
    fn equality() {
        assert_eq!(Ident::key("a"), Ident::key("a"));
        assert_ne!(Ident::key("a"), Ident::key("b"));
        assert_eq!(Ident::index(0), Ident::index(0));
        assert_ne!(Ident::index(0), Ident::index(1));
        assert_ne!(Ident::key("0"), Ident::index(0));
    }

    #[test]
    fn hash() {
        let mut set = HashSet::new();
        set.insert(Ident::key("a"));
        set.insert(Ident::key("a"));
        set.insert(Ident::index(0));
        assert_eq!(set.len(), 2);
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize_key() {
            let json = serde_json::to_string(&Ident::key("foo")).unwrap();
            assert_eq!(json, "\"foo\"");
        }

        #[test]
        fn serialize_index() {
            let json = serde_json::to_string(&Ident::index(7)).unwrap();
            assert_eq!(json, "7");
        }

        #[test]
        fn deserialize_key() {
            let i: Ident = serde_json::from_str("\"foo\"").unwrap();
            assert_eq!(i, Ident::key("foo"));
        }

        #[test]
        fn deserialize_index() {
            let i: Ident = serde_json::from_str("7").unwrap();
            assert_eq!(i, Ident::index(7));
        }
    }
}
