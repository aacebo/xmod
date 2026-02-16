use crate::Segment;

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Path(Vec<Segment>);

impl Path {
    pub fn parse(src: &str) -> Self {
        let mut items = vec![];

        for item in src.split("/") {
            items.push(Segment::parse(item));
        }

        Self(items)
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
        Self::parse(value)
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self::parse(&value)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, segment) in self.0.iter().enumerate() {
            write!(f, "{}", segment)?;

            if i < self.0.len() - 1 {
                write!(f, "/")?;
            }
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
