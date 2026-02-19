use std::sync::Arc;

use crate::{Object, ToValue, Value};

pub trait Array: Send + Sync {
    fn name(&self) -> &str;
    fn type_id(&self) -> std::any::TypeId;
    fn len(&self) -> usize;
    fn items(&self) -> ArrayIter<'_>;
    fn index(&self, i: usize) -> Option<&dyn ToValue>;

    fn is_empty(&self) -> bool {
        self.len() == 0
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

    fn items(&self) -> ArrayIter<'_> {
        ArrayIter::new(self.iter().map(|v| v as &dyn ToValue))
    }

    fn index(&self, i: usize) -> Option<&dyn ToValue> {
        self.get(i).map(|v| v as &dyn ToValue)
    }
}

impl<T: Clone + ToValue + 'static> ToValue for Vec<T> {
    fn to_value(&self) -> Value {
        Value::from_array(self.iter().map(|v| v.to_value()).collect::<Vec<_>>())
    }
}

impl<T: Clone + ToValue + 'static> ToValue for [T] {
    fn to_value(&self) -> Value {
        Value::from_array(self.iter().map(|v| v.to_value()).collect::<Vec<_>>())
    }
}

impl<T: Clone + ToValue + 'static> ToValue for &[T] {
    fn to_value(&self) -> Value {
        Value::from_array(self.iter().map(|v| v.to_value()).collect::<Vec<_>>())
    }
}

impl From<Vec<Value>> for Object {
    fn from(value: Vec<Value>) -> Self {
        Self::Array(Arc::new(value))
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::Object(Object::from(value))
    }
}

impl std::fmt::Debug for dyn Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_list();

        for v in self.items() {
            dbg.entry(&v.to_value());
        }

        dbg.finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let len = self.len();
        let mut seq = serializer.serialize_seq(Some(len))?;

        for item in self.items() {
            seq.serialize_element(&item.to_value())?;
        }

        seq.end()
    }
}

pub struct ArrayIter<'a>(Box<dyn Iterator<Item = &'a dyn ToValue> + 'a>);

impl<'a> ArrayIter<'a> {
    pub fn new(iter: impl Iterator<Item = &'a dyn ToValue> + 'a) -> Self {
        Self(Box::new(iter))
    }
}

impl<'a> Iterator for ArrayIter<'a> {
    type Item = &'a dyn ToValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn sample_array() -> Vec<Value> {
        vec![valueof!(1_i32), valueof!(true), valueof!("hello")]
    }

    #[test]
    fn name() {
        let a = sample_array();
        assert_eq!(a.name(), "Vec");
    }

    #[test]
    fn type_id() {
        let a = sample_array();
        assert_eq!(a.type_id(), std::any::TypeId::of::<Vec<Value>>());
    }

    #[test]
    fn len() {
        let a = sample_array();
        assert_eq!(a.len(), 3);
    }

    #[test]
    fn is_empty() {
        let empty: Vec<Value> = vec![];
        assert!(empty.is_empty());
        assert!(!sample_array().is_empty());
    }

    #[test]
    fn items() {
        let a = sample_array();
        let items: Vec<_> = a.items().collect();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].to_value().to_i32(), 1);
        assert_eq!(items[1].to_value().to_bool(), true);
        assert_eq!(items[2].to_value().as_str(), "hello");
    }

    #[test]
    fn index() {
        let a = sample_array();
        let v = a.index(0).unwrap();
        assert_eq!(v.to_value().to_i32(), 1);

        let v = a.index(2).unwrap();
        assert_eq!(v.to_value().as_str(), "hello");

        assert!(a.index(99).is_none());
    }

    #[test]
    fn to_value() {
        let a = sample_array();
        let v = a.to_value();
        assert!(v.is_object());
        assert!(v.is_array());
    }

    #[test]
    fn vec_i32_to_value() {
        let v = vec![1i32, 2, 3].to_value();
        assert!(v.is_array());
        let arr = v.as_array();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.index(0).unwrap().to_value().to_i32(), 1);
        assert_eq!(arr.index(1).unwrap().to_value().to_i32(), 2);
        assert_eq!(arr.index(2).unwrap().to_value().to_i32(), 3);
    }

    #[test]
    fn vec_bool_to_value() {
        let v = vec![true, false].to_value();
        assert!(v.is_array());
        let arr = v.as_array();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.index(0).unwrap().to_value().to_bool(), true);
        assert_eq!(arr.index(1).unwrap().to_value().to_bool(), false);
    }

    #[test]
    fn vec_string_to_value() {
        let v = vec![String::from("a"), String::from("b")].to_value();
        assert!(v.is_array());
        let arr = v.as_array();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.index(0).unwrap().to_value().as_str(), "a");
        assert_eq!(arr.index(1).unwrap().to_value().as_str(), "b");
    }

    #[test]
    fn vec_empty_to_value() {
        let v = Vec::<i32>::new().to_value();
        assert!(v.is_array());
        assert_eq!(v.as_array().len(), 0);
    }

    #[test]
    fn vec_nested_to_value() {
        let v = vec![vec![1i32, 2], vec![3, 4]].to_value();
        assert!(v.is_array());
        let outer = v.as_array();
        assert_eq!(outer.len(), 2);
        let inner = outer.index(0).unwrap().to_value();
        assert!(inner.is_array());
        assert_eq!(inner.as_array().len(), 2);
        assert_eq!(inner.as_array().index(0).unwrap().to_value().to_i32(), 1);
    }
}
