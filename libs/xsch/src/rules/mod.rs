mod one_of;
mod required;

pub use one_of::*;
pub use required::*;

use crate::{Context, ValidError, Validate};

#[derive(Debug, Clone)]
pub enum Rule {
    OneOf(OneOf),
    Required(Required),
}

impl Rule {
    pub fn name(&self) -> &str {
        match self {
            Self::OneOf(_) => "one_of",
            Self::Required(_) => "required",
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Rule {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Required(v) => v.serialize(s),
            Self::OneOf(v) => v.serialize(s),
        }
    }
}

#[cfg(feature = "serde")]
impl Rule {
    pub fn deserialize_by_name<'de, A>(name: &str, map: &mut A) -> Result<Option<Self>, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        match name {
            "required" => Ok(Some(Self::Required(map.next_value()?))),
            "one_of" => Ok(Some(Self::OneOf(map.next_value()?))),
            _ => Ok(None),
        }
    }
}

impl Validate for Rule {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::OneOf(v) => v.validate(ctx),
            Self::Required(v) => v.validate(ctx),
        }
    }
}
