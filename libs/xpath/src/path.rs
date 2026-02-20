use crate::{Ident, ParseError};

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Path(Vec<Ident>);

impl Path {
    pub fn parse(src: &str) -> Result<Self, ParseError> {
        if src.is_empty() {
            return Ok(Self(vec![]));
        }

        let mut items = vec![];

        for item in src.split("/") {
            if item.is_empty() {
                return Err("path segments cannot be empty".into());
            }

            items.push(Ident::parse(item));
        }

        Ok(Self(items))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn last(&self) -> Option<&Ident> {
        self.0.last()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Ident> {
        self.0.iter()
    }

    pub fn push(&mut self, ident: impl Into<Ident>) -> &mut Self {
        self.0.push(ident.into());
        self
    }

    pub fn pop(&mut self) -> Option<Ident> {
        self.0.pop()
    }

    pub fn child(&self, ident: impl Into<Ident>) -> Self {
        let mut path = self.clone();
        path.0.push(ident.into());
        path
    }

    pub fn peer(&self, ident: impl Into<Ident>) -> Self {
        let mut path = self.clone();
        path.0.pop();
        path.0.push(ident.into());
        path
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self::parse(value).unwrap()
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self::parse(&value).unwrap()
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, ident) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "/")?;
            }

            write!(f, "{}", ident)?;
        }

        Ok(())
    }
}

impl std::ops::Index<usize> for Path {
    type Output = Ident;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Path {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Path {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(d)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let path = Path::parse("a/b/c").unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], Ident::Key("a".into()));
        assert_eq!(path[1], Ident::Key("b".into()));
        assert_eq!(path[2], Ident::Key("c".into()));
    }

    #[test]
    fn parse_mixed() {
        let path = Path::parse("users/0/name").unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], Ident::Key("users".into()));
        assert_eq!(path[1], Ident::Index(0));
        assert_eq!(path[2], Ident::Key("name".into()));
    }

    #[test]
    fn parse_single() {
        let path = Path::parse("key").unwrap();
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], Ident::Key("key".into()));
    }

    #[test]
    fn parse_empty() {
        let path = Path::parse("").unwrap();
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
    }

    #[test]
    fn parse_double_slash() {
        assert!(Path::parse("a//b").is_err());
    }

    #[test]
    fn parse_leading_slash() {
        assert!(Path::parse("/a").is_err());
    }

    #[test]
    fn parse_trailing_slash() {
        assert!(Path::parse("a/").is_err());
    }

    #[test]
    fn from_str() {
        let path: Path = "a/b".into();
        assert_eq!(path.len(), 2);
    }

    #[test]
    fn from_string() {
        let path: Path = String::from("a/b").into();
        assert_eq!(path.len(), 2);
    }

    #[test]
    fn len() {
        assert_eq!(Path::parse("a/b/c").unwrap().len(), 3);
        assert_eq!(Path::parse("a").unwrap().len(), 1);
        assert_eq!(Path::parse("").unwrap().len(), 0);
    }

    #[test]
    fn is_empty() {
        assert!(Path::default().is_empty());
        assert!(Path::parse("").unwrap().is_empty());
        assert!(!Path::parse("a").unwrap().is_empty());
    }

    #[test]
    fn last() {
        assert_eq!(
            Path::parse("a/b/c").unwrap().last(),
            Some(&Ident::Key("c".into()))
        );
        assert_eq!(
            Path::parse("users/0").unwrap().last(),
            Some(&Ident::Index(0))
        );
        assert_eq!(Path::parse("").unwrap().last(), None);
    }

    #[test]
    fn index() {
        let path = Path::parse("x/1/y").unwrap();
        assert_eq!(path[0], Ident::Key("x".into()));
        assert_eq!(path[1], Ident::Index(1));
        assert_eq!(path[2], Ident::Key("y".into()));
    }

    #[test]
    fn display() {
        assert_eq!(Path::parse("a/b/c").unwrap().to_string(), "a/b/c");
        assert_eq!(
            Path::parse("users/0/name").unwrap().to_string(),
            "users/0/name"
        );
        assert_eq!(Path::parse("key").unwrap().to_string(), "key");
    }

    #[test]
    fn display_empty() {
        assert_eq!(Path::parse("").unwrap().to_string(), "");
        assert_eq!(Path::default().to_string(), "");
    }

    #[test]
    fn eq() {
        assert_eq!(Path::parse("a/0/b").unwrap(), Path::parse("a/0/b").unwrap());
        assert_ne!(Path::parse("a/0").unwrap(), Path::parse("a/1").unwrap());
    }

    #[test]
    fn default() {
        let path = Path::default();
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
        assert_eq!(path.last(), None);
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;

        #[test]
        fn serialize() {
            let path = Path::parse("users/0/name").unwrap();
            let json = serde_json::to_string(&path).unwrap();
            assert_eq!(json, r#""users/0/name""#);
        }

        #[test]
        fn serialize_empty() {
            let path = Path::default();
            let json = serde_json::to_string(&path).unwrap();
            assert_eq!(json, r#""""#);
        }

        #[test]
        fn deserialize() {
            let path: Path = serde_json::from_str(r#""users/0/name""#).unwrap();
            assert_eq!(path.len(), 3);
            assert_eq!(path[0], Ident::Key("users".into()));
            assert_eq!(path[1], Ident::Index(0));
            assert_eq!(path[2], Ident::Key("name".into()));
        }

        #[test]
        fn deserialize_empty() {
            let path: Path = serde_json::from_str(r#""""#).unwrap();
            assert!(path.is_empty());
        }

        #[test]
        fn deserialize_invalid() {
            let result = serde_json::from_str::<Path>(r#""a//b""#);
            assert!(result.is_err());
        }

        #[test]
        fn roundtrip() {
            let original = Path::parse("a/1/b/2").unwrap();
            let json = serde_json::to_string(&original).unwrap();
            let restored: Path = serde_json::from_str(&json).unwrap();
            assert_eq!(original, restored);
        }
    }
}
