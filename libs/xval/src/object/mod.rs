mod arrays;
mod structs;
mod tuples;

pub use arrays::*;
pub use structs::*;
pub use tuples::*;

use std::sync::Arc;

use crate::{ToValue, Value};

#[derive(Clone)]
pub enum Object {
    Struct(Arc<dyn Struct>),
    Array(Arc<dyn Array>),
    Tuple(Arc<dyn Tuple>),
}

impl Object {
    pub fn from_struct<T: Struct + 'static>(value: T) -> Self {
        Self::Struct(Arc::new(value))
    }

    pub fn from_array<T: Array + 'static>(value: T) -> Self {
        Self::Array(Arc::new(value))
    }

    pub fn from_tuple<T: Tuple + 'static>(value: T) -> Self {
        Self::Tuple(Arc::new(value))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(_))
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Struct(v) => v.name(),
            Self::Array(v) => v.name(),
            Self::Tuple(v) => v.name(),
        }
    }

    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::Struct(v) => v.type_id(),
            Self::Array(v) => v.type_id(),
            Self::Tuple(v) => v.type_id(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Struct(v) => v.len(),
            Self::Array(v) => v.len(),
            Self::Tuple(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_struct(&self) -> &Arc<dyn Struct> {
        match self {
            Self::Struct(v) => v,
            v => panic!("expected Struct, received {}", v.name()),
        }
    }

    pub fn as_array(&self) -> &Arc<dyn Array> {
        match self {
            Self::Array(v) => v,
            v => panic!("expected Array, received {}", v.name()),
        }
    }

    pub fn as_tuple(&self) -> &Arc<dyn Tuple> {
        match self {
            Self::Tuple(v) => v,
            v => panic!("expected Tuple, received {}", v.name()),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Struct(a), Self::Struct(b)) => {
                if a.len() != b.len() {
                    return false;
                }

                a.items().all(|(k, v)| {
                    b.field(k.clone())
                        .is_some_and(|bv| v.to_value() == bv.to_value())
                })
            }
            (Self::Array(a), Self::Array(b)) => {
                if a.len() != b.len() {
                    return false;
                }

                a.items()
                    .zip(b.items())
                    .all(|(av, bv)| av.to_value() == bv.to_value())
            }
            (Self::Tuple(a), Self::Tuple(b)) => {
                if a.len() != b.len() {
                    return false;
                }

                a.items()
                    .zip(b.items())
                    .all(|(av, bv)| av.to_value() == bv.to_value())
            }
            _ => false,
        }
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Struct(v) => write!(f, "{:#?}", v),
            Self::Array(v) => write!(f, "{:#?}", v),
            Self::Tuple(v) => write!(f, "{:#?}", v),
        }
    }
}

impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Self::Object(value)
    }
}

impl Value {
    pub fn from_struct<T: Struct + 'static>(value: T) -> Self {
        Self::Object(Object::from_struct(value))
    }

    pub fn from_array<T: Array + 'static>(value: T) -> Self {
        Self::Object(Object::from_array(value))
    }

    pub fn from_tuple<T: Tuple + 'static>(value: T) -> Self {
        Self::Object(Object::from_tuple(value))
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl ToValue for Object {
    fn to_value(&self) -> Value {
        Value::Object(self.clone())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Object::Struct(s) => s.as_ref().serialize(serializer),
            Object::Array(a) => a.as_ref().serialize(serializer),
            Object::Tuple(t) => t.as_ref().serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ObjectVisitor)
    }
}

#[cfg(feature = "serde")]
struct ObjectVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for ObjectVisitor {
    type Value = Object;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map or sequence")
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        use std::collections::HashMap;

        let mut result = HashMap::new();

        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            use crate::Ident;

            result.insert(Ident::key(&key), value);
        }

        Ok(Object::Struct(Arc::new(result)))
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut result = Vec::new();

        while let Some(value) = seq.next_element::<Value>()? {
            result.push(value);
        }

        Ok(Object::Array(Arc::new(result)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::*;

    fn sample_struct() -> HashMap<Ident, Value> {
        let mut map = HashMap::new();
        map.insert(Ident::key("a"), valueof!(1_i32));
        map.insert(Ident::key("b"), valueof!("hello"));
        map
    }

    fn sample_array() -> Vec<Value> {
        vec![valueof!(1_i32), valueof!(true), valueof!("hello")]
    }

    fn sample_tuple() -> (Value, Value, Value) {
        (valueof!(1_i32), valueof!(true), valueof!("hello"))
    }

    mod objects {
        use std::sync::Arc;

        use super::*;

        #[test]
        fn is_struct() {
            let obj = Object::Struct(Arc::new(sample_struct()));
            assert!(obj.is_struct());
            assert!(!obj.is_array());
            assert!(!obj.is_tuple());
        }

        #[test]
        fn is_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            assert!(obj.is_array());
            assert!(!obj.is_struct());
            assert!(!obj.is_tuple());
        }

        #[test]
        fn is_tuple() {
            let obj = Object::Tuple(Arc::new(sample_tuple()));
            assert!(obj.is_tuple());
            assert!(!obj.is_struct());
            assert!(!obj.is_array());
        }

        #[test]
        fn name() {
            let s = Object::Struct(Arc::new(sample_struct()));
            assert_eq!(s.name(), "HashMap");

            let a = Object::Array(Arc::new(sample_array()));
            assert_eq!(a.name(), "Vec");

            let t = Object::Tuple(Arc::new(sample_tuple()));
            assert_eq!(t.name(), "Tuple3");
        }

        #[test]
        fn type_id() {
            let s = Object::Struct(Arc::new(sample_struct()));
            assert_eq!(s.type_id(), std::any::TypeId::of::<HashMap<Ident, Value>>());

            let a = Object::Array(Arc::new(sample_array()));
            assert_eq!(a.type_id(), std::any::TypeId::of::<Vec<Value>>());

            let t = Object::Tuple(Arc::new(sample_tuple()));
            assert_eq!(t.type_id(), std::any::TypeId::of::<(Value, Value, Value)>());
        }

        #[test]
        fn len() {
            let s = Object::from_struct(sample_struct());
            assert_eq!(s.len(), 2);

            let a = Object::from_array(sample_array());
            assert_eq!(a.len(), 3);

            let t = Object::from_tuple(sample_tuple());
            assert_eq!(t.len(), 3);
        }

        #[test]
        fn is_empty() {
            let a = Object::from_array(sample_array());
            assert!(!a.is_empty());

            let empty = Object::from_array(Vec::<Value>::new());
            assert!(empty.is_empty());
        }

        #[test]
        fn as_struct() {
            let obj = Object::Struct(Arc::new(sample_struct()));
            let s = obj.as_struct();
            assert_eq!(s.len(), 2);
        }

        #[test]
        #[should_panic(expected = "expected Struct")]
        fn as_struct_panics_on_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            obj.as_struct();
        }

        #[test]
        fn as_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            let a = obj.as_array();
            assert_eq!(a.len(), 3);
        }

        #[test]
        #[should_panic(expected = "expected Array")]
        fn as_array_panics_on_struct() {
            let obj = Object::Struct(Arc::new(sample_struct()));
            obj.as_array();
        }

        #[test]
        fn as_tuple() {
            let obj = Object::Tuple(Arc::new(sample_tuple()));
            let t = obj.as_tuple();
            assert_eq!(t.len(), 3);
        }

        #[test]
        #[should_panic(expected = "expected Tuple")]
        fn as_tuple_panics_on_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            obj.as_tuple();
        }

        #[test]
        fn debug_struct() {
            let obj = Object::Struct(Arc::new(sample_struct()));
            let dbg = format!("{:?}", obj);
            assert!(dbg.contains("a"));
            assert!(dbg.contains("b"));
        }

        #[test]
        fn debug_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            let dbg = format!("{:?}", obj);
            assert!(dbg.starts_with('['));
            assert!(dbg.ends_with(']'));
        }

        #[test]
        fn debug_tuple() {
            let obj = Object::Tuple(Arc::new(sample_tuple()));
            let dbg = format!("{:?}", obj);
            assert!(dbg.starts_with("Tuple3"));
        }

        #[test]
        fn clone() {
            let inner = Arc::new(sample_array());
            let obj = Object::Array(inner.clone());
            let cloned = obj.clone();
            assert_eq!(
                obj.as_array().as_ref() as *const dyn Array,
                cloned.as_array().as_ref() as *const dyn Array
            );
        }

        #[test]
        fn eq_structs_same_content() {
            let a = Object::from_struct(sample_struct());
            let b = Object::from_struct(sample_struct());
            assert_eq!(a, b);
        }

        #[test]
        fn ne_structs_different_values() {
            let mut m1 = HashMap::new();
            m1.insert(Ident::key("x"), valueof!(1_i32));

            let mut m2 = HashMap::new();
            m2.insert(Ident::key("x"), valueof!(2_i32));

            assert_ne!(Object::from_struct(m1), Object::from_struct(m2));
        }

        #[test]
        fn ne_structs_different_keys() {
            let mut m1 = HashMap::new();
            m1.insert(Ident::key("a"), valueof!(1_i32));

            let mut m2 = HashMap::new();
            m2.insert(Ident::key("b"), valueof!(1_i32));

            assert_ne!(Object::from_struct(m1), Object::from_struct(m2));
        }

        #[test]
        fn ne_structs_different_lengths() {
            let mut m1 = HashMap::new();
            m1.insert(Ident::key("a"), valueof!(1_i32));

            let mut m2 = HashMap::new();
            m2.insert(Ident::key("a"), valueof!(1_i32));
            m2.insert(Ident::key("b"), valueof!(2_i32));

            assert_ne!(Object::from_struct(m1), Object::from_struct(m2));
        }

        #[test]
        fn eq_arrays_same_content() {
            let a = Object::from_array(sample_array());
            let b = Object::from_array(sample_array());
            assert_eq!(a, b);
        }

        #[test]
        fn ne_arrays_different_elements() {
            let a = Object::from_array(vec![valueof!(1_i32)]);
            let b = Object::from_array(vec![valueof!(2_i32)]);
            assert_ne!(a, b);
        }

        #[test]
        fn ne_arrays_different_lengths() {
            let a = Object::from_array(vec![valueof!(1_i32)]);
            let b = Object::from_array(vec![valueof!(1_i32), valueof!(2_i32)]);
            assert_ne!(a, b);
        }

        #[test]
        fn eq_tuples_same_content() {
            let a = Object::from_tuple(sample_tuple());
            let b = Object::from_tuple(sample_tuple());
            assert_eq!(a, b);
        }

        #[test]
        fn ne_tuples_different_elements() {
            let a = Object::from_tuple((valueof!(1_i32), valueof!(true)));
            let b = Object::from_tuple((valueof!(2_i32), valueof!(true)));
            assert_ne!(a, b);
        }

        #[test]
        fn ne_struct_vs_array() {
            let s = Object::from_struct(sample_struct());
            let a = Object::from_array(sample_array());
            assert_ne!(s, a);
        }

        #[test]
        fn eq_empty_arrays() {
            let a = Object::from_array(Vec::<Value>::new());
            let b = Object::from_array(Vec::<Value>::new());
            assert_eq!(a, b);
        }

        #[test]
        fn eq_nested_objects() {
            let inner1 = Object::from_array(vec![valueof!(1_i32)]);
            let inner2 = Object::from_array(vec![valueof!(1_i32)]);
            let a = Object::from_array(vec![Value::Object(inner1)]);
            let b = Object::from_array(vec![Value::Object(inner2)]);
            assert_eq!(a, b);
        }

        #[test]
        fn ne_nested_objects() {
            let inner1 = Object::from_array(vec![valueof!(1_i32)]);
            let inner2 = Object::from_array(vec![valueof!(2_i32)]);
            let a = Object::from_array(vec![Value::Object(inner1)]);
            let b = Object::from_array(vec![Value::Object(inner2)]);
            assert_ne!(a, b);
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn from_object_for_value() {
            let obj = Object::from(sample_array());
            let v = Value::from(obj);
            assert!(v.is_object());
            assert!(v.is_array());
        }

        #[test]
        fn from_hashmap_for_object() {
            let obj = Object::from(sample_struct());
            assert!(obj.is_struct());
            assert_eq!(obj.as_struct().len(), 2);
        }

        #[test]
        fn from_vec_for_object() {
            let obj = Object::from(sample_array());
            assert!(obj.is_array());
            assert_eq!(obj.as_array().len(), 3);
        }

        #[test]
        fn from_tuple_for_object() {
            let obj = Object::from(sample_tuple());
            assert!(obj.is_tuple());
            assert_eq!(obj.as_tuple().len(), 3);
        }

        #[test]
        fn from_hashmap_for_value() {
            let v = Value::from(sample_struct());
            assert!(v.is_struct());
        }

        #[test]
        fn from_vec_for_value() {
            let v = Value::from(sample_array());
            assert!(v.is_array());
        }

        #[test]
        fn from_tuple_for_value() {
            let v = Value::from(sample_tuple());
            assert!(v.is_tuple());
        }

        #[test]
        fn value_from_struct() {
            let v = Value::from_struct(sample_struct());
            assert!(v.is_struct());
            assert_eq!(v.as_struct().len(), 2);
        }

        #[test]
        fn value_from_array() {
            let v = Value::from_array(sample_array());
            assert!(v.is_array());
            assert_eq!(v.as_array().len(), 3);
        }

        #[test]
        fn value_from_tuple() {
            let v = Value::from_tuple(sample_tuple());
            assert!(v.is_tuple());
            assert_eq!(v.as_tuple().len(), 3);
        }

        #[test]
        fn display() {
            let s = Object::from(sample_struct());
            assert_eq!(s.to_string(), "HashMap");

            let a = Object::from(sample_array());
            assert_eq!(a.to_string(), "Vec");
        }

        #[test]
        fn to_value_object() {
            let obj = Object::from(sample_array());
            let v = obj.to_value();
            assert!(v.is_array());
        }
    }

    #[cfg(feature = "serde")]
    mod serde {
        use std::sync::Arc;

        use super::*;

        #[test]
        fn serialize_struct() {
            let obj = Object::Struct(Arc::new(sample_struct()));
            let json: serde_json::Value = serde_json::to_value(&obj).unwrap();
            assert!(json.is_object());
            assert_eq!(json["a"], 1);
            assert_eq!(json["b"], "hello");
        }

        #[test]
        fn serialize_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            let json = serde_json::to_string(&obj).unwrap();
            assert_eq!(json, "[1,true,\"hello\"]");
        }

        #[test]
        fn serialize_tuple() {
            let obj = Object::Tuple(Arc::new(sample_tuple()));
            let json = serde_json::to_string(&obj).unwrap();
            assert_eq!(json, "[1,true,\"hello\"]");
        }

        #[test]
        fn deserialize_struct() {
            let obj: Object = serde_json::from_str(r#"{"x": 42, "y": "world"}"#).unwrap();
            assert!(obj.is_struct());
            let s = obj.as_struct();
            assert_eq!(s.len(), 2);
            assert_eq!(s.field("x".into()).unwrap().to_value().to_i8(), 42);
            assert_eq!(s.field("y".into()).unwrap().to_value().as_str(), "world");
        }

        #[test]
        fn deserialize_array() {
            let obj: Object = serde_json::from_str("[10, false, \"test\"]").unwrap();
            assert!(obj.is_array());
            let a = obj.as_array();
            assert_eq!(a.len(), 3);
            assert_eq!(a.index(0).unwrap().to_value().to_i8(), 10);
            assert_eq!(a.index(1).unwrap().to_value().to_bool(), false);
            assert_eq!(a.index(2).unwrap().to_value().as_str(), "test");
        }

        #[test]
        fn round_trip_struct() {
            let obj = Object::Struct(Arc::new(sample_struct()));
            let json = serde_json::to_string(&obj).unwrap();
            let deserialized: Object = serde_json::from_str(&json).unwrap();
            assert!(deserialized.is_struct());
            let s = deserialized.as_struct();
            assert_eq!(s.len(), 2);
            assert_eq!(s.field("a".into()).unwrap().to_value().to_i8(), 1);
        }

        #[test]
        fn round_trip_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            let json = serde_json::to_string(&obj).unwrap();
            let deserialized: Object = serde_json::from_str(&json).unwrap();
            assert!(deserialized.is_array());
            let a = deserialized.as_array();
            assert_eq!(a.len(), 3);
            assert_eq!(a.index(0).unwrap().to_value().to_i8(), 1);
            assert_eq!(a.index(1).unwrap().to_value().to_bool(), true);
            assert_eq!(a.index(2).unwrap().to_value().as_str(), "hello");
        }
    }
}
