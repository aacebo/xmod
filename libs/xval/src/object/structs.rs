use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use crate::{Ident, Object, ToValue, Value};

pub trait Struct: Send + Sync {
    fn name(&self) -> &str;
    fn type_id(&self) -> std::any::TypeId;
    fn len(&self) -> usize;
    fn items(&self) -> StructIter<'_>;
    fn field(&self, ident: Ident) -> Option<&dyn ToValue>;

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
        StructIter::new(self.iter().map(|(k, v)| (k.clone(), v as &dyn ToValue)))
    }

    fn field(&self, ident: Ident) -> Option<&dyn ToValue> {
        self.get(&ident).map(|v| v as &dyn ToValue)
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

impl<T: Clone + ToValue + 'static> ToValue for HashMap<Ident, T> {
    fn to_value(&self) -> Value {
        Value::from_struct(
            self.iter()
                .map(|(k, v)| (k.clone(), v.to_value()))
                .collect::<HashMap<Ident, Value>>(),
        )
    }
}

impl Struct for BTreeMap<Ident, Value> {
    fn name(&self) -> &str {
        "BTreeMap"
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn items(&self) -> StructIter<'_> {
        StructIter::new(self.iter().map(|(k, v)| (k.clone(), v as &dyn ToValue)))
    }

    fn field(&self, ident: Ident) -> Option<&dyn ToValue> {
        self.get(&ident).map(|v| v as &dyn ToValue)
    }
}

impl From<BTreeMap<Ident, Value>> for Object {
    fn from(value: BTreeMap<Ident, Value>) -> Self {
        Self::Struct(Arc::new(value))
    }
}

impl From<BTreeMap<Ident, Value>> for Value {
    fn from(value: BTreeMap<Ident, Value>) -> Self {
        Self::Object(Object::from(value))
    }
}

impl<T: Clone + ToValue + 'static> ToValue for BTreeMap<Ident, T> {
    fn to_value(&self) -> Value {
        Value::from_struct(
            self.iter()
                .map(|(k, v)| (k.clone(), v.to_value()))
                .collect::<BTreeMap<Ident, Value>>(),
        )
    }
}

impl std::fmt::Debug for dyn Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct(self.name());

        for (k, v) in self.items() {
            dbg.field(&k.to_string(), &v.to_value());
        }

        dbg.finish()
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
            map.serialize_entry(&ident.to_string(), &value.to_value())?;
        }

        map.end()
    }
}

pub struct StructIter<'a>(Box<dyn Iterator<Item = (Ident, &'a dyn ToValue)> + 'a>);

impl<'a> StructIter<'a> {
    pub fn new<T: Iterator<Item = (Ident, &'a dyn ToValue)> + 'a>(iter: T) -> Self {
        Self(Box::new(iter))
    }
}

impl<'a> Iterator for StructIter<'a> {
    type Item = (Ident, &'a dyn ToValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, const N: usize> From<&'a [(Ident, Value); N]> for StructIter<'a> {
    fn from(value: &'a [(Ident, Value); N]) -> Self {
        Self::new(
            value
                .iter()
                .map(|(ident, value)| (ident.clone(), value as &dyn ToValue)),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ext::StructExt;
    use crate::*;

    fn sample_struct() -> HashMap<Ident, Value> {
        let mut map = HashMap::new();
        map.insert(Ident::key("a"), valueof!(1_i32));
        map.insert(Ident::key("b"), valueof!("hello"));
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
        let v = Struct::field(&s, "a".into()).unwrap();
        assert_eq!(v.to_value().to_i32(), 1);

        let v = Struct::field(&s, "b".into()).unwrap();
        assert_eq!(v.to_value().as_str(), "hello");

        assert!(Struct::field(&s, "missing".into()).is_none());
    }

    #[test]
    fn to_value() {
        let s = sample_struct();
        let v = s.to_value();
        assert!(v.is_object());
        assert!(v.is_struct());
    }

    #[test]
    fn hashmap_i32_to_value() {
        let mut map = HashMap::new();
        map.insert(Ident::key("x"), 10i32);
        map.insert(Ident::key("y"), 20i32);
        let v = map.to_value();
        assert!(v.is_struct());
        let s = v.as_struct();
        assert_eq!(s.len(), 2);
        assert_eq!(s.get("x").unwrap().to_value().to_i32(), 10);
        assert_eq!(s.get("y").unwrap().to_value().to_i32(), 20);
    }

    #[test]
    fn hashmap_string_to_value() {
        let mut map = HashMap::new();
        map.insert(Ident::key("name"), String::from("alice"));
        let v = map.to_value();
        assert!(v.is_struct());
        let s = v.as_struct();
        assert_eq!(s.get("name").unwrap().to_value().as_str(), "alice");
    }

    #[test]
    fn hashmap_empty_to_value() {
        let map: HashMap<Ident, i32> = HashMap::new();
        let v = map.to_value();
        assert!(v.is_struct());
        assert_eq!(v.as_struct().len(), 0);
    }
}
