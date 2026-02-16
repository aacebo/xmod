use crate::{ParseError, Segment};

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Path(Vec<Segment>);

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

            items.push(Segment::parse(item));
        }

        Ok(Self(items))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn last(&self) -> Option<&Segment> {
        self.0.last()
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
        for (i, segment) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "/")?;
            }

            write!(f, "{}", segment)?;
        }

        Ok(())
    }
}

impl std::ops::Index<usize> for Path {
    type Output = Segment;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let path = Path::parse("a/b/c").unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], Segment::Key("a".into()));
        assert_eq!(path[1], Segment::Key("b".into()));
        assert_eq!(path[2], Segment::Key("c".into()));
    }

    #[test]
    fn parse_mixed() {
        let path = Path::parse("users/0/name").unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], Segment::Key("users".into()));
        assert_eq!(path[1], Segment::Index(0));
        assert_eq!(path[2], Segment::Key("name".into()));
    }

    #[test]
    fn parse_single() {
        let path = Path::parse("key").unwrap();
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], Segment::Key("key".into()));
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
            Some(&Segment::Key("c".into()))
        );
        assert_eq!(
            Path::parse("users/0").unwrap().last(),
            Some(&Segment::Index(0))
        );
        assert_eq!(Path::parse("").unwrap().last(), None);
    }

    #[test]
    fn index() {
        let path = Path::parse("x/1/y").unwrap();
        assert_eq!(path[0], Segment::Key("x".into()));
        assert_eq!(path[1], Segment::Index(1));
        assert_eq!(path[2], Segment::Key("y".into()));
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
}
