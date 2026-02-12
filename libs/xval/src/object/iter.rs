use crate::{AsValue, Ident, Value};

pub struct StructIter<'a>(std::collections::hash_map::Iter<'a, Ident, Value>);

impl<'a> Iterator for StructIter<'a> {
    type Item = (&'a Ident, &'a dyn AsValue);

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some((ident, value)) => Some((ident, value as &dyn AsValue)),
        }
    }
}

impl<'a> From<std::collections::hash_map::Iter<'a, Ident, Value>> for StructIter<'a> {
    fn from(value: std::collections::hash_map::Iter<'a, Ident, Value>) -> Self {
        Self(value)
    }
}

pub struct ArrayIter<'a>(std::slice::Iter<'a, Value>);

impl<'a> Iterator for ArrayIter<'a> {
    type Item = &'a dyn AsValue;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(value) => Some(value as &dyn AsValue),
        }
    }
}

impl<'a> From<std::slice::Iter<'a, Value>> for ArrayIter<'a> {
    fn from(value: std::slice::Iter<'a, Value>) -> Self {
        Self(value)
    }
}
