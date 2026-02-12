use std::collections::HashMap;
use std::sync::Arc;

use crate::{Ident, Value};

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
    fn fields(&self) -> Vec<Ident>;
    fn field(&self, ident: &Ident) -> Option<&dyn crate::ToValue>;
}

pub trait Array: Send + Sync {
    fn name(&self) -> &str;
    fn type_id(&self) -> std::any::TypeId;
    fn len(&self) -> usize;
    fn index(&self, i: usize) -> Option<&dyn crate::ToValue>;
}

impl Struct for HashMap<Ident, Value> {
    fn name(&self) -> &str {
        "HashMap"
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<HashMap<Ident, Value>>()
    }

    fn fields(&self) -> Vec<Ident> {
        self.keys().cloned().collect()
    }

    fn field(&self, ident: &Ident) -> Option<&dyn crate::ToValue> {
        self.get(ident).map(|v| v as &dyn crate::ToValue)
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
        Vec::len(self)
    }

    fn index(&self, i: usize) -> Option<&dyn crate::ToValue> {
        self.get(i).map(|v| v as &dyn crate::ToValue)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Object {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Struct(v) => {
                use serde::ser::SerializeMap;
                let fields = v.fields();
                let mut map = serializer.serialize_map(Some(fields.len()))?;
                for ident in &fields {
                    if let Some(val) = v.field(ident) {
                        map.serialize_entry(&ident.to_string(), &val.to_value_ref())?;
                    }
                }
                map.end()
            }
            Self::Array(v) => {
                use serde::ser::SerializeSeq;
                let len = v.len();
                let mut seq = serializer.serialize_seq(Some(len))?;
                for i in 0..len {
                    if let Some(val) = v.index(i) {
                        seq.serialize_element(&val.to_value_ref())?;
                    }
                }
                seq.end()
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Object {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
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
        let mut result = HashMap::new();
        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            result.insert(Ident::key(&key), value);
        }
        Ok(Object::Struct(Arc::new(result)))
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(
        self,
        mut seq: A,
    ) -> Result<Self::Value, A::Error> {
        let mut result = Vec::new();
        while let Some(value) = seq.next_element::<Value>()? {
            result.push(value);
        }
        Ok(Object::Array(Arc::new(result)))
    }
}
