mod iter;
#[cfg(feature = "serde")]
mod serial;

use std::collections::HashMap;
use std::sync::Arc;

use crate::{AsValue, Ident, Value};

#[derive(Clone)]
pub enum Object {
    Struct(Arc<dyn Struct>),
    Array(Arc<dyn Array>),
}

impl Object {
    pub fn name(&self) -> &str {
        match self {
            Self::Struct(v) => v.name(),
            Self::Array(v) => v.name(),
        }
    }

    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::Struct(v) => v.type_id(),
            Self::Array(v) => v.type_id(),
        }
    }
}

pub trait Struct: Send + Sync {
    fn name(&self) -> &str;
    fn type_id(&self) -> std::any::TypeId;
    fn iter(&self) -> iter::StructIter<'_>;
    fn field(&self, ident: &Ident) -> Option<&dyn AsValue>;
}

pub trait Array: Send + Sync {
    fn name(&self) -> &str;
    fn type_id(&self) -> std::any::TypeId;
    fn len(&self) -> usize;
    fn items(&self) -> iter::ArrayIter<'_>;
    fn index(&self, i: usize) -> Option<&dyn AsValue>;
}

impl Struct for HashMap<Ident, Value> {
    fn name(&self) -> &str {
        "HashMap"
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn iter(&self) -> iter::StructIter<'_> {
        self.iter().into()
    }

    fn field(&self, ident: &Ident) -> Option<&dyn AsValue> {
        self.get(ident).map(|v| v as &dyn AsValue)
    }
}

impl Array for Vec<Value> {
    fn name(&self) -> &str {
        "Vec"
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Vec<Value>>()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn items(&self) -> iter::ArrayIter<'_> {
        self.iter().into()
    }

    fn index(&self, i: usize) -> Option<&dyn AsValue> {
        self.get(i).map(|v| v as &dyn AsValue)
    }
}
