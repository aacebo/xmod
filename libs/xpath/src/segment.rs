#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
