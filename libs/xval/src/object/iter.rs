use crate::{AsValue, Ident};

pub struct KeysIter<'a>(Box<dyn Iterator<Item = &'a Ident> + 'a>);

impl<'a> KeysIter<'a> {
    pub fn new(iter: impl Iterator<Item = &'a Ident> + 'a) -> Self {
        Self(Box::new(iter))
    }
}

impl<'a> Iterator for KeysIter<'a> {
    type Item = &'a Ident;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct StructIter<'a>(Box<dyn Iterator<Item = (&'a Ident, &'a dyn AsValue)> + 'a>);

impl<'a> StructIter<'a> {
    pub fn new(iter: impl Iterator<Item = (&'a Ident, &'a dyn AsValue)> + 'a) -> Self {
        Self(Box::new(iter))
    }
}

impl<'a> Iterator for StructIter<'a> {
    type Item = (&'a Ident, &'a dyn AsValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct ArrayIter<'a>(Box<dyn Iterator<Item = &'a dyn AsValue> + 'a>);

impl<'a> ArrayIter<'a> {
    pub fn new(iter: impl Iterator<Item = &'a dyn AsValue> + 'a) -> Self {
        Self(Box::new(iter))
    }
}

impl<'a> Iterator for ArrayIter<'a> {
    type Item = &'a dyn AsValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
