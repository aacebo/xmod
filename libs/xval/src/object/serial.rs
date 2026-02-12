use std::{collections::HashMap, sync::Arc};

use crate::{Array, Ident, Object, Struct, Value};

impl serde::Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Object::Struct(s) => s.as_ref().serialize(serializer),
            Object::Array(a) => a.as_ref().serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ObjectVisitor)
    }
}

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

impl serde::Serialize for dyn Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let len = self.len();
        let mut seq = serializer.serialize_seq(Some(len))?;

        for item in self.items() {
            seq.serialize_element(&item.as_value())?;
        }

        seq.end()
    }
}

struct ObjectVisitor;

impl<'de> serde::de::Visitor<'de> for ObjectVisitor {
    type Value = Object;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map or sequence")
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut result = HashMap::new();

        while let Some((key, value)) = map.next_entry::<String, Value>()? {
            let ident = match key.parse::<usize>() {
                Ok(i) => Ident::index(i),
                Err(_) => Ident::key(&key),
            };

            result.insert(ident, value);
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
