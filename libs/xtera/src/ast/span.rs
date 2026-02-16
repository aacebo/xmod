/// Byte-offset range in the source string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
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
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let s = Span::new(0, 5);
        assert_eq!(s.start, 0);
        assert_eq!(s.end, 5);
    }

    #[test]
    fn merge() {
        let a = Span::new(2, 5);
        let b = Span::new(8, 12);
        let merged = a.merge(b);
        assert_eq!(merged, Span::new(2, 12));
    }

    #[test]
    fn from_range() {
        let s: Span = (3..7).into();
        assert_eq!(s, Span::new(3, 7));
    }

    #[test]
    fn into_range() {
        let r: std::ops::Range<usize> = Span::new(1, 4).into();
        assert_eq!(r, 1..4);
    }

    #[test]
    fn display() {
        assert_eq!(Span::new(0, 10).to_string(), "0..10");
    }
}
