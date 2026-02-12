use std::{collections::HashMap, sync::Arc};

use crate::{AsValue, Ident, Object, Value};

pub trait Struct: Send + Sync {
    fn name(&self) -> &str;
    fn type_id(&self) -> std::any::TypeId;
    fn len(&self) -> usize;
    fn items(&self) -> StructIter<'_>;
    fn field(&self, ident: Ident) -> Option<&dyn AsValue>;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Struct for HashMap<Ident, Value> {
    fn name(&self) -> &str {
        "HashMap"
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn items(&self) -> StructIter<'_> {
        StructIter::new(self.iter().map(|(k, v)| (k.clone(), v as &dyn AsValue)))
    }

    fn field(&self, ident: Ident) -> Option<&dyn AsValue> {
        self.get(&ident).map(|v| v as &dyn AsValue)
    }
}

impl From<HashMap<Ident, Value>> for Object {
    fn from(value: HashMap<Ident, Value>) -> Self {
        Self::Struct(Arc::new(value))
    }
}

impl From<HashMap<Ident, Value>> for Value {
    fn from(value: HashMap<Ident, Value>) -> Self {
        Self::Object(Object::from(value))
    }
}

impl AsValue for HashMap<Ident, Value> {
    fn as_value(&self) -> Value {
        Value::Object(Object::Struct(Arc::new(self.clone())))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn Struct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let iter = self.items();
        let mut map = serializer.serialize_map(None)?;

        for (ident, value) in iter {
            map.serialize_entry(&ident.to_string(), &value.as_value())?;
        }

        map.end()
    }
}

pub struct StructIter<'a>(Box<dyn Iterator<Item = (Ident, &'a dyn AsValue)> + 'a>);

impl<'a> StructIter<'a> {
    pub fn new<T: Iterator<Item = (Ident, &'a dyn AsValue)> + 'a>(iter: T) -> Self {
        Self(Box::new(iter))
    }
}

impl<'a> Iterator for StructIter<'a> {
    type Item = (Ident, &'a dyn AsValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, const N: usize> From<&'a [(Ident, Value); N]> for StructIter<'a> {
    fn from(value: &'a [(Ident, Value); N]) -> Self {
        Self::new(
            value
                .iter()
                .map(|(ident, value)| (ident.clone(), value as &dyn AsValue)),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::*;

    fn sample_struct() -> HashMap<Ident, Value> {
        let mut map = HashMap::new();
        map.insert(Ident::key("a"), Value::from_i32(1));
        map.insert(Ident::key("b"), Value::from_str("hello"));
        map
    }

    #[test]
    fn name() {
        let s = sample_struct();
        assert_eq!(s.name(), "HashMap");
    }

    #[test]
    fn type_id() {
        let s = sample_struct();
        assert_eq!(s.type_id(), std::any::TypeId::of::<HashMap<Ident, Value>>());
    }

    #[test]
    fn len() {
        let s = sample_struct();
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn is_empty() {
        let empty: HashMap<Ident, Value> = HashMap::new();
        assert!(empty.is_empty());
        assert!(!sample_struct().is_empty());
    }

    #[test]
    fn keys() {
        let s = sample_struct();
        let mut keys: Vec<_> = s.keys().collect();
        keys.sort_by_key(|k| k.to_string());
        assert_eq!(keys.len(), 2);
        assert_eq!(keys[0].to_string(), "a");
        assert_eq!(keys[1].to_string(), "b");
    }

    #[test]
    fn items() {
        let s = sample_struct();
        let items: Vec<_> = s.items().collect();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn field() {
        let s = sample_struct();
        let v = s.field("a".into()).unwrap();
        assert_eq!(v.as_value().to_i32(), 1);

        let v = s.field("b".into()).unwrap();
        assert_eq!(v.as_value().as_str(), "hello");

        assert!(s.field("missing".into()).is_none());
    }

    #[test]
    fn as_value() {
        let s = sample_struct();
        let v = s.as_value();
        assert!(v.is_object());
        assert!(v.is_struct());
    }
}
