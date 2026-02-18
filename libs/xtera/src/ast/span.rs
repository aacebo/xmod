use std::sync::Arc;

/// Byte-offset range in the source string, with a shared
/// reference to the full source so that `Display` can show
/// the original text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub src: Arc<str>,
}

impl Span {
    pub fn new(start: usize, end: usize, src: Arc<str>) -> Self {
        Self { start, end, src }
    }

    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            src: self.src.clone(),
        }
    }
}

impl From<Span> for std::ops::Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.src[self.start..self.end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn src() -> Arc<str> {
        Arc::from("hello world")
    }

    #[test]
    fn new() {
        let s = Span::new(0, 5, src());
        assert_eq!(s.start, 0);
        assert_eq!(s.end, 5);
    }

    #[test]
    fn merge() {
        let a = Span::new(2, 5, src());
        let b = Span::new(8, 11, src());
        let merged = a.merge(&b);
        assert_eq!(merged, Span::new(2, 11, src()));
    }

    #[test]
    fn into_range() {
        let r: std::ops::Range<usize> = Span::new(1, 4, src()).into();
        assert_eq!(r, 1..4);
    }

    #[test]
    fn display() {
        assert_eq!(Span::new(0, 5, src()).to_string(), "hello");
        assert_eq!(Span::new(6, 11, src()).to_string(), "world");
    }
}
