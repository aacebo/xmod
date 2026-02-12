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
