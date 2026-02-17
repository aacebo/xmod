mod bool;
mod ident;
pub mod num;
mod object;
mod string;

pub use bool::*;
pub use ident::*;
pub use num::*;
pub use object::*;
pub use string::*;

#[cfg(feature = "derive")]
pub mod derive {
    pub use xval_derive::*;
}

/// A trait for types that can produce a [`Value`] from a shared reference.
pub trait AsValue {
    fn as_value(&self) -> Value;
}

/// A dynamically-typed value that can hold a boolean or any numeric type.
#[derive(Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged)
)]
pub enum Value {
    Null,
    Bool(Bool),
    Number(Number),
    String(Str),
    Object(Object),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    pub fn as_bool(&self) -> &Bool {
        match self {
            Self::Bool(v) => v,
            v => panic!("expected Bool, received {}", std::any::type_name_of_val(v)),
        }
    }

    pub fn as_number(&self) -> &Number {
        match self {
            Self::Number(v) => v,
            v => panic!(
                "expected Number, received {}",
                std::any::type_name_of_val(v)
            ),
        }
    }

    pub fn as_string(&self) -> &Str {
        match self {
            Self::String(v) => v,
            v => panic!(
                "expected String, received {}",
                std::any::type_name_of_val(v)
            ),
        }
    }

    pub fn as_object(&self) -> &Object {
        match self {
            Self::Object(v) => v,
            v => panic!(
                "expected Object, received {}",
                std::any::type_name_of_val(v)
            ),
        }
    }

    pub fn to_bool(&self) -> bool {
        self.as_bool().to_bool()
    }

    pub fn as_str(&self) -> &str {
        self.as_string().as_str()
    }

    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::Null => panic!("attempt to call Value::type_id on Value::Null"),
            Self::Bool(v) => v.type_id(),
            Self::Number(v) => v.type_id(),
            Self::String(v) => v.type_id(),
            Self::Object(v) => v.type_id(),
        }
    }
}

impl Value {
    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Object(v) if v.is_struct())
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Object(v) if v.is_array())
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Object(v) if v.is_tuple())
    }

    pub fn as_struct(&self) -> &std::sync::Arc<dyn Struct> {
        self.as_object().as_struct()
    }

    pub fn as_array(&self) -> &std::sync::Arc<dyn Array> {
        self.as_object().as_array()
    }

    pub fn as_tuple(&self) -> &std::sync::Arc<dyn Tuple> {
        self.as_object().as_tuple()
    }
}

impl Value {
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_float())
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_int())
    }

    pub fn is_uint(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_uint())
    }

    pub fn as_float(&self) -> &Float {
        self.as_number().as_float()
    }

    pub fn as_int(&self) -> &Int {
        self.as_number().as_int()
    }

    pub fn as_uint(&self) -> &UInt {
        self.as_number().as_uint()
    }
}

impl Value {
    pub fn is_f32(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_f32())
    }

    pub fn is_f64(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_f64())
    }

    pub fn to_f32(&self) -> f32 {
        self.as_number().to_f32()
    }

    pub fn to_f64(&self) -> f64 {
        self.as_number().to_f64()
    }
}

impl Value {
    pub fn is_i8(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_i8())
    }

    pub fn is_i16(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_i16())
    }

    pub fn is_i32(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_i32())
    }

    pub fn is_i64(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_i64())
    }

    pub fn to_i8(&self) -> i8 {
        self.as_number().to_i8()
    }

    pub fn to_i16(&self) -> i16 {
        self.as_number().to_i16()
    }

    pub fn to_i32(&self) -> i32 {
        self.as_number().to_i32()
    }

    pub fn to_i64(&self) -> i64 {
        self.as_number().to_i64()
    }
}

impl Value {
    pub fn is_u8(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_u8())
    }

    pub fn is_u16(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_u16())
    }

    pub fn is_u32(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_u32())
    }

    pub fn is_u64(&self) -> bool {
        matches!(self, Self::Number(v) if v.is_u64())
    }

    pub fn to_u8(&self) -> u8 {
        self.as_number().to_u8()
    }

    pub fn to_u16(&self) -> u16 {
        self.as_number().to_u16()
    }

    pub fn to_u32(&self) -> u32 {
        self.as_number().to_u32()
    }

    pub fn to_u64(&self) -> u64 {
        self.as_number().to_u64()
    }
}

impl Value {
    pub fn get(&self, path: &xpath::Path) -> Option<Self> {
        let mut value = self.clone();

        for segment in path.iter() {
            let next = match segment {
                xpath::Segment::Key(v) => value.as_object().as_struct().field(v.as_str().into()),
                xpath::Segment::Index(v) => value.as_object().as_array().index(*v),
            };

            value = match next {
                None => return None,
                Some(v) => v.as_value(),
            };
        }

        Some(value)
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "<null>"),
            Self::Bool(v) => write!(f, "{:#?}", v),
            Self::Number(v) => write!(f, "{:#?}", v),
            Self::String(v) => write!(f, "{:#?}", v),
            Self::Object(v) => write!(f, "{:#?}", v),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "<null>"),
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Object(v) => write!(f, "{}", v),
        }
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Null => other.is_null(),
            Self::Bool(v) => other.is_bool() && v.eq(other.as_bool()),
            Self::Number(v) => other.is_number() && v.eq(other.as_number()),
            Self::String(v) => other.is_string() && v.eq(other.as_string()),
            Self::Object(v) => other.is_object() && v.eq(other.as_object()),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl AsValue for Value {
    fn as_value(&self) -> Value {
        self.clone()
    }
}

impl<T: AsValue> AsValue for Box<T> {
    fn as_value(&self) -> Value {
        self.as_ref().as_value()
    }
}

impl<T: AsValue> AsValue for std::rc::Rc<T> {
    fn as_value(&self) -> Value {
        self.as_ref().as_value()
    }
}

impl<T: AsValue> AsValue for std::sync::Arc<T> {
    fn as_value(&self) -> Value {
        self.as_ref().as_value()
    }
}

impl<T: AsValue> AsValue for std::cell::RefCell<T> {
    fn as_value(&self) -> Value {
        self.borrow().as_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_predicates() {
        let null = Value::Null;
        assert!(null.is_null());
        assert!(!null.is_bool());
        assert!(!null.is_number());
        assert!(!null.is_string());
        assert!(!null.is_object());

        let b = Value::from_bool(true);
        assert!(b.is_bool());
        assert!(!b.is_null());
        assert!(!b.is_number());
        assert!(!b.is_string());

        let n = Value::from_i32(1);
        assert!(n.is_number());
        assert!(!n.is_null());
        assert!(!n.is_bool());
        assert!(!n.is_string());

        let s = Value::from_str("hello");
        assert!(s.is_string());
        assert!(!s.is_null());
        assert!(!s.is_bool());
        assert!(!s.is_number());
    }

    #[test]
    fn as_bool() {
        let v = Value::from_bool(true);
        assert_eq!(v.to_bool(), true);
    }

    #[test]
    fn as_number() {
        let v = Value::from_i32(42);
        assert_eq!(*v.as_number(), Number::from_i32(42));
    }

    #[test]
    #[should_panic(expected = "expected Bool")]
    fn as_bool_panics_on_mismatch() {
        Value::from_i32(1).as_bool();
    }

    #[test]
    #[should_panic(expected = "expected Number")]
    fn as_number_panics_on_mismatch() {
        Value::from_bool(true).as_number();
    }

    #[test]
    fn as_str() {
        let v = Value::from_str("hello");
        assert_eq!(v.as_str(), "hello");
    }

    #[test]
    #[should_panic(expected = "expected Str")]
    fn as_str_panics_on_mismatch() {
        Value::from_bool(true).as_str();
    }

    #[test]
    fn debug_null() {
        assert_eq!(format!("{:?}", Value::Null), "<null>");
    }

    #[test]
    fn eq_null() {
        assert_eq!(Value::Null, Value::Null);
        assert_ne!(Value::Null, Value::from_bool(true));
        assert_ne!(Value::Null, Value::from_i32(0));
        assert_ne!(Value::Null, Value::from_str(""));
    }

    #[test]
    fn clone_null() {
        let v = Value::Null;
        let cloned = v.clone();
        assert!(cloned.is_null());
        assert_eq!(v, cloned);
    }

    #[test]
    #[should_panic(expected = "Value::Null")]
    fn type_id_panics_on_null() {
        Value::Null.type_id();
    }

    #[test]
    #[should_panic(expected = "expected Bool")]
    fn as_bool_panics_on_null() {
        Value::Null.as_bool();
    }

    #[test]
    #[should_panic(expected = "expected Number")]
    fn as_number_panics_on_null() {
        Value::Null.as_number();
    }

    #[test]
    #[should_panic(expected = "expected String")]
    fn as_string_panics_on_null() {
        Value::Null.as_string();
    }

    #[test]
    #[should_panic(expected = "expected Object")]
    fn as_object_panics_on_null() {
        Value::Null.as_object();
    }

    #[test]
    fn display() {
        assert_eq!(Value::Null.to_string(), "<null>");
        assert_eq!(Value::from_bool(true).to_string(), "true");
        assert_eq!(Value::from_i32(42).to_string(), "42");
        assert_eq!(Value::from_f64(3.14).to_string(), "3.14");
        assert_eq!(Value::from_str("hello").to_string(), "hello");
    }

    #[test]
    fn type_id() {
        assert_eq!(
            Value::from_bool(true).type_id(),
            std::any::TypeId::of::<bool>()
        );
        assert_eq!(
            Value::from_f32(1.0).type_id(),
            std::any::TypeId::of::<f32>()
        );
        assert_eq!(
            Value::from_f64(1.0).type_id(),
            std::any::TypeId::of::<f64>()
        );
        assert_eq!(Value::from_i32(1).type_id(), std::any::TypeId::of::<i32>());
        assert_eq!(Value::from_u32(1).type_id(), std::any::TypeId::of::<u32>());
        assert_eq!(
            Value::from_str("hello").type_id(),
            std::any::TypeId::of::<String>()
        );
    }

    mod get {
        use std::collections::HashMap;

        use super::*;

        #[test]
        fn struct_field() {
            let mut map = HashMap::new();
            map.insert(Ident::key("a"), Value::from_i32(1));
            map.insert(Ident::key("b"), Value::from_str("hello"));
            let v = Value::from_struct(map);

            let path = xpath::Path::parse("a").unwrap();
            let result = v.get(&path).unwrap();
            assert_eq!(result.to_i32(), 1);

            let path = xpath::Path::parse("b").unwrap();
            let result = v.get(&path).unwrap();
            assert_eq!(result.as_str(), "hello");
        }

        #[test]
        fn array_index() {
            let arr = vec![
                Value::from_i32(10),
                Value::from_bool(true),
                Value::from_str("world"),
            ];
            let v = Value::from_array(arr);

            let path = xpath::Path::parse("0").unwrap();
            assert_eq!(v.get(&path).unwrap().to_i32(), 10);

            let path = xpath::Path::parse("1").unwrap();
            assert_eq!(v.get(&path).unwrap().to_bool(), true);

            let path = xpath::Path::parse("2").unwrap();
            assert_eq!(v.get(&path).unwrap().as_str(), "world");
        }

        #[test]
        fn nested_struct_to_array() {
            let arr = vec![Value::from_i32(42), Value::from_i32(99)];
            let mut map = HashMap::new();
            map.insert(Ident::key("items"), Value::from_array(arr));
            let v = Value::from_struct(map);

            let path = xpath::Path::parse("items/0").unwrap();
            assert_eq!(v.get(&path).unwrap().to_i32(), 42);

            let path = xpath::Path::parse("items/1").unwrap();
            assert_eq!(v.get(&path).unwrap().to_i32(), 99);
        }

        #[test]
        fn nested_array_to_struct() {
            let mut m1 = HashMap::new();
            m1.insert(Ident::key("name"), Value::from_str("alice"));
            let mut m2 = HashMap::new();
            m2.insert(Ident::key("name"), Value::from_str("bob"));
            let arr = vec![Value::from_struct(m1), Value::from_struct(m2)];
            let v = Value::from_array(arr);

            let path = xpath::Path::parse("0/name").unwrap();
            assert_eq!(v.get(&path).unwrap().as_str(), "alice");

            let path = xpath::Path::parse("1/name").unwrap();
            assert_eq!(v.get(&path).unwrap().as_str(), "bob");
        }

        #[test]
        fn missing_key() {
            let mut map = HashMap::new();
            map.insert(Ident::key("a"), Value::from_i32(1));
            let v = Value::from_struct(map);

            let path = xpath::Path::parse("z").unwrap();
            assert!(v.get(&path).is_none());
        }

        #[test]
        fn missing_index() {
            let arr = vec![Value::from_i32(1)];
            let v = Value::from_array(arr);

            let path = xpath::Path::parse("5").unwrap();
            assert!(v.get(&path).is_none());
        }

        #[test]
        fn empty_path() {
            let v = Value::from_i32(42);
            let path = xpath::Path::parse("").unwrap();
            let result = v.get(&path).unwrap();
            assert_eq!(result.to_i32(), 42);
        }
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn serialize() {
            assert_eq!(
                serde_json::to_string(&Value::from_bool(true)).unwrap(),
                "true"
            );
            assert_eq!(serde_json::to_string(&Value::from_i32(42)).unwrap(), "42");
            assert_eq!(
                serde_json::to_string(&Value::from_f64(3.14)).unwrap(),
                "3.14"
            );
            assert_eq!(
                serde_json::to_string(&Value::from_str("hello")).unwrap(),
                "\"hello\""
            );
        }

        #[test]
        fn deserialize() {
            let v: Value = serde_json::from_str("true").unwrap();
            assert_eq!(v.to_bool(), true);

            let v: Value = serde_json::from_str("false").unwrap();
            assert_eq!(v.to_bool(), false);

            let v: Value = serde_json::from_str("\"hello\"").unwrap();
            assert_eq!(v.as_str(), "hello");
        }

        #[test]
        fn deserialize_object_map() {
            let v: Value = serde_json::from_str(r#"{"a": 1, "b": "hello"}"#).unwrap();
            assert!(v.is_object());
        }

        #[test]
        fn deserialize_object_array() {
            let v: Value = serde_json::from_str("[1, true, \"hello\"]").unwrap();
            assert!(v.is_object());
        }

        #[test]
        fn serialize_null() {
            assert_eq!(serde_json::to_string(&Value::Null).unwrap(), "null");
        }

        #[test]
        fn deserialize_null() {
            let v: Value = serde_json::from_str("null").unwrap();
            assert!(v.is_null());
        }

        #[test]
        fn serialize_object_array() {
            let arr = vec![
                Value::from_i32(1),
                Value::from_bool(true),
                Value::from_str("a"),
            ];
            let v = Value::Object(Object::Array(std::sync::Arc::new(arr)));
            let json = serde_json::to_string(&v).unwrap();
            assert_eq!(json, "[1,true,\"a\"]");
        }
    }
}
