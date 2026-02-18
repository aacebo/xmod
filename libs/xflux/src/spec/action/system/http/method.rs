use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
}

impl HttpMethod {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Head => "HEAD",
            Self::Options => "OPTIONS",
            Self::Connect => "CONNECT",
            Self::Trace => "TRACE",
        }
    }

    #[inline]
    pub const fn as_lowercase(&self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Put => "put",
            Self::Patch => "patch",
            Self::Delete => "delete",
            Self::Head => "head",
            Self::Options => "options",
            Self::Connect => "connect",
            Self::Trace => "trace",
        }
    }

    #[inline]
    pub const fn as_bytes(&self) -> &'static [u8] {
        self.as_str().as_bytes()
    }

    #[inline]
    pub const fn is_get(&self) -> bool {
        matches!(self, Self::Get)
    }

    #[inline]
    pub const fn is_post(&self) -> bool {
        matches!(self, Self::Post)
    }

    #[inline]
    pub const fn is_put(&self) -> bool {
        matches!(self, Self::Put)
    }

    #[inline]
    pub const fn is_patch(&self) -> bool {
        matches!(self, Self::Patch)
    }

    #[inline]
    pub const fn is_delete(&self) -> bool {
        matches!(self, Self::Delete)
    }

    #[inline]
    pub const fn is_head(&self) -> bool {
        matches!(self, Self::Head)
    }

    #[inline]
    pub const fn is_options(&self) -> bool {
        matches!(self, Self::Options)
    }

    #[inline]
    pub const fn is_connect(&self) -> bool {
        matches!(self, Self::Connect)
    }

    #[inline]
    pub const fn is_trace(&self) -> bool {
        matches!(self, Self::Trace)
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for HttpMethod {
    type Err = InvalidHttpMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            "CONNECT" => Ok(Self::Connect),
            "TRACE" => Ok(Self::Trace),
            _ => Err(InvalidHttpMethod(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidHttpMethod(pub String);

impl fmt::Display for InvalidHttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid HTTP method: {}", self.0)
    }
}

impl std::error::Error for InvalidHttpMethod {}
