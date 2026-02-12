mod array;
mod iter;
#[cfg(feature = "serde")]
mod serial;
mod r#struct;

pub use array::*;
pub use r#struct::*;

use std::sync::Arc;

use crate::{AsValue, Value};

#[derive(Clone)]
pub enum Object {
    Struct(Arc<dyn Struct>),
    Array(Arc<dyn Array>),
}

impl Object {
    pub fn from_struct<T: Struct + 'static>(value: T) -> Self {
        Self::Struct(Arc::new(value))
    }

    pub fn from_array<T: Array + 'static>(value: T) -> Self {
        Self::Array(Arc::new(value))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

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
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Struct(s) => {
                let mut dbg = f.debug_map();

                for (k, v) in s.items() {
                    dbg.entry(k, &v.as_value());
                }

                dbg.finish()
            }
            Self::Array(a) => {
                let mut dbg = f.debug_list();

                for v in a.items() {
                    dbg.entry(&v.as_value());
                }

                dbg.finish()
            }
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
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsValue for Object {
    fn as_value(&self) -> Value {
        Value::Object(self.clone())
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

    fn sample_array() -> Vec<Value> {
        vec![
            Value::from_i32(1),
            Value::from_bool(true),
            Value::from_str("hello"),
        ]
    }

    mod objects {
        use std::sync::Arc;

        use super::*;

        #[test]
        fn is_struct() {
            let obj = Object::Struct(Arc::new(sample_struct()));
            assert!(obj.is_struct());
            assert!(!obj.is_array());
        }

        #[test]
        fn is_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            assert!(obj.is_array());
            assert!(!obj.is_struct());
        }

        #[test]
        fn name() {
            let s = Object::Struct(Arc::new(sample_struct()));
            assert_eq!(s.name(), "HashMap");

            let a = Object::Array(Arc::new(sample_array()));
            assert_eq!(a.name(), "Vec");
        }

        #[test]
        fn type_id() {
            let s = Object::Struct(Arc::new(sample_struct()));
            assert_eq!(s.type_id(), std::any::TypeId::of::<HashMap<Ident, Value>>());

            let a = Object::Array(Arc::new(sample_array()));
            assert_eq!(a.type_id(), std::any::TypeId::of::<Vec<Value>>());
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
        fn clone() {
            let inner = Arc::new(sample_array());
            let obj = Object::Array(inner.clone());
            let cloned = obj.clone();
            assert_eq!(
                obj.as_array().as_ref() as *const dyn Array,
                cloned.as_array().as_ref() as *const dyn Array
            );
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
        fn display() {
            let s = Object::from(sample_struct());
            assert_eq!(s.to_string(), "HashMap");

            let a = Object::from(sample_array());
            assert_eq!(a.to_string(), "Vec");
        }

        #[test]
        fn as_value_object() {
            let obj = Object::from(sample_array());
            let v = obj.as_value();
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
        fn deserialize_struct() {
            let obj: Object = serde_json::from_str(r#"{"x": 42, "y": "world"}"#).unwrap();
            assert!(obj.is_struct());
            let s = obj.as_struct();
            assert_eq!(s.len(), 2);
            assert_eq!(s.field(&Ident::key("x")).unwrap().as_value().to_i8(), 42);
            assert_eq!(
                s.field(&Ident::key("y")).unwrap().as_value().as_str(),
                "world"
            );
        }

        #[test]
        fn deserialize_array() {
            let obj: Object = serde_json::from_str("[10, false, \"test\"]").unwrap();
            assert!(obj.is_array());
            let a = obj.as_array();
            assert_eq!(a.len(), 3);
            assert_eq!(a.index(0).unwrap().as_value().to_i8(), 10);
            assert_eq!(a.index(1).unwrap().as_value().to_bool(), false);
            assert_eq!(a.index(2).unwrap().as_value().as_str(), "test");
        }

        #[test]
        fn round_trip_struct() {
            let obj = Object::Struct(Arc::new(sample_struct()));
            let json = serde_json::to_string(&obj).unwrap();
            let deserialized: Object = serde_json::from_str(&json).unwrap();
            assert!(deserialized.is_struct());
            let s = deserialized.as_struct();
            assert_eq!(s.len(), 2);
            assert_eq!(s.field(&Ident::key("a")).unwrap().as_value().to_i8(), 1);
        }

        #[test]
        fn round_trip_array() {
            let obj = Object::Array(Arc::new(sample_array()));
            let json = serde_json::to_string(&obj).unwrap();
            let deserialized: Object = serde_json::from_str(&json).unwrap();
            assert!(deserialized.is_array());
            let a = deserialized.as_array();
            assert_eq!(a.len(), 3);
            assert_eq!(a.index(0).unwrap().as_value().to_i8(), 1);
            assert_eq!(a.index(1).unwrap().as_value().to_bool(), true);
            assert_eq!(a.index(2).unwrap().as_value().as_str(), "hello");
        }
    }
}
