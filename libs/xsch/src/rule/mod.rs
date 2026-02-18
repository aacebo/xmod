mod equals;
mod fields;
mod items;
mod max;
mod min;
mod options;
#[cfg(feature = "regex")]
mod pattern;
mod required;

pub use equals::*;
pub use fields::*;
pub use items::*;
pub use max::*;
pub use min::*;
pub use options::*;
#[cfg(feature = "regex")]
pub use pattern::*;
pub use required::*;

use crate::{Context, ValidError, Validate};

#[derive(Debug, Clone)]
pub enum Rule {
    Equals(Equals),
    Required(Required),
    Options(Options),
    Min(Min),
    Max(Max),
    Items(Items),
    Fields(Fields),
    #[cfg(feature = "regex")]
    Pattern(Pattern),
}

impl Rule {
    pub fn key(&self) -> &str {
        match self {
            Self::Equals(_) => Equals::KEY,
            Self::Options(_) => Options::KEY,
            Self::Required(_) => Required::KEY,
            Self::Min(_) => Min::KEY,
            Self::Max(_) => Max::KEY,
            Self::Items(_) => Items::KEY,
            Self::Fields(_) => Fields::KEY,
            #[cfg(feature = "regex")]
            Self::Pattern(_) => Pattern::KEY,
        }
    }

    pub fn as_equals(&self) -> Option<&Equals> {
        match self {
            Self::Equals(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_equals_mut(&mut self) -> Option<&mut Equals> {
        match self {
            Self::Equals(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_required(&self) -> Option<&Required> {
        match self {
            Self::Required(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_required_mut(&mut self) -> Option<&mut Required> {
        match self {
            Self::Required(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_options(&self) -> Option<&Options> {
        match self {
            Self::Options(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_options_mut(&mut self) -> Option<&mut Options> {
        match self {
            Self::Options(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_min(&self) -> Option<&Min> {
        match self {
            Self::Min(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_min_mut(&mut self) -> Option<&mut Min> {
        match self {
            Self::Min(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_max(&self) -> Option<&Max> {
        match self {
            Self::Max(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_max_mut(&mut self) -> Option<&mut Max> {
        match self {
            Self::Max(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_items(&self) -> Option<&Items> {
        match self {
            Self::Items(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_items_mut(&mut self) -> Option<&mut Items> {
        match self {
            Self::Items(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_fields(&self) -> Option<&Fields> {
        match self {
            Self::Fields(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_fields_mut(&mut self) -> Option<&mut Fields> {
        match self {
            Self::Fields(v) => Some(v),
            _ => None,
        }
    }

    #[cfg(feature = "regex")]
    pub fn as_pattern(&self) -> Option<&Pattern> {
        match self {
            Self::Pattern(v) => Some(v),
            _ => None,
        }
    }

    #[cfg(feature = "regex")]
    pub fn as_pattern_mut(&mut self) -> Option<&mut Pattern> {
        match self {
            Self::Pattern(v) => Some(v),
            _ => None,
        }
    }
}

impl Validate for Rule {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::Equals(v) => v.validate(ctx),
            Self::Options(v) => v.validate(ctx),
            Self::Required(v) => v.validate(ctx),
            Self::Min(v) => v.validate(ctx),
            Self::Max(v) => v.validate(ctx),
            Self::Items(v) => v.validate(ctx),
            Self::Fields(v) => v.validate(ctx),
            #[cfg(feature = "regex")]
            Self::Pattern(v) => v.validate(ctx),
        }
    }
}

#[cfg(feature = "serde")]
impl Rule {
    pub fn serialize_entry<S: serde::ser::SerializeMap>(
        &self,
        map: &mut S,
    ) -> Result<(), S::Error> {
        match self {
            Self::Equals(v) => map.serialize_entry(Equals::KEY, v),
            Self::Required(v) => map.serialize_entry(Required::KEY, v),
            Self::Options(v) => map.serialize_entry(Options::KEY, v),
            Self::Min(v) => map.serialize_entry(Min::KEY, v),
            Self::Max(v) => map.serialize_entry(Max::KEY, v),
            Self::Items(v) => map.serialize_entry(Items::KEY, v),
            Self::Fields(v) => map.serialize_entry(Fields::KEY, v),
            #[cfg(feature = "regex")]
            Self::Pattern(v) => map.serialize_entry(Pattern::KEY, v),
        }
    }

    pub fn deserialize_entry<'de, A: serde::de::MapAccess<'de>>(
        key: &str,
        map: &mut A,
    ) -> Result<Option<Self>, A::Error> {
        match key {
            Equals::KEY => Ok(Some(Self::Equals(map.next_value()?))),
            Required::KEY => Ok(Some(Self::Required(map.next_value()?))),
            Options::KEY => Ok(Some(Self::Options(map.next_value()?))),
            Min::KEY => Ok(Some(Self::Min(map.next_value()?))),
            Max::KEY => Ok(Some(Self::Max(map.next_value()?))),
            Items::KEY => Ok(Some(Self::Items(map.next_value()?))),
            Fields::KEY => Ok(Some(Self::Fields(map.next_value()?))),
            #[cfg(feature = "regex")]
            Pattern::KEY => Ok(Some(Self::Pattern(map.next_value()?))),
            _ => {
                let _ = map.next_value::<serde::de::IgnoredAny>()?;
                Ok(None)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RuleSet(Vec<Rule>);

impl RuleSet {
    pub fn exists(&self, key: &str) -> bool {
        self.0.iter().any(|r| r.key() == key)
    }

    pub fn get(&self, key: &str) -> Option<&Rule> {
        self.0.iter().find(|r| r.key() == key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Rule> {
        self.0.iter_mut().find(|r| r.key() == key)
    }

    pub fn add(mut self, rule: Rule) -> Self {
        self.0.push(rule);
        self
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.0.extend(other.0);
        self
    }
}

impl std::ops::Deref for RuleSet {
    type Target = [Rule];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self.0)
    }
}

impl Validate for RuleSet {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let mut next = ctx.clone();
        let mut error = ValidError::new(ctx.path.clone()).name(&ctx.name).build();

        for rule in &self.0 {
            next.name = rule.key().to_string();
            next.value = match rule.validate(&next) {
                Ok(v) => v,
                Err(err) => {
                    error.errors.push(err);
                    continue;
                }
            };
        }

        if !error.errors.is_empty() {
            return Err(error);
        }

        Ok(next.value)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RuleSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for rule in &self.0 {
            rule.serialize_entry(&mut map)?;
        }

        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RuleSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};

        struct RuleSetVisitor;

        impl<'de> Visitor<'de> for RuleSetVisitor {
            type Value = RuleSet;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a map of rule names to rule values")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                use serde::de::Error;

                let mut rules = vec![];

                while let Some(key) = map.next_key::<String>()? {
                    if let Some(rule) = Rule::deserialize_entry(&key, &mut map)? {
                        rules.push(rule);
                    } else {
                        return Err(A::Error::custom(format!("rule '{}' not found", &key)));
                    }
                }

                Ok(RuleSet(rules))
            }
        }

        deserializer.deserialize_map(RuleSetVisitor)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    mod serde {
        use xval::AsValue;

        use crate::*;

        #[test]
        fn serialize_empty() {
            let rs = RuleSet::default();
            let json = serde_json::to_string(&rs).unwrap();
            assert_eq!(json, "{}");
        }

        #[test]
        fn serialize_required() {
            let rs = RuleSet::default().add(Required::new(true).into());
            let json = serde_json::to_string(&rs).unwrap();
            assert_eq!(json, r#"{"required":true}"#);
        }

        #[test]
        fn serialize_multiple_rules() {
            let rs = RuleSet::default().add(Required::new(true).into()).add(
                Options::from(vec![1i32.as_value(), "test".as_value(), true.as_value()]).into(),
            );
            let json = serde_json::to_string(&rs).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["required"], serde_json::json!(true));
            assert_eq!(v["options"], serde_json::json!([1, "test", true]));
        }

        #[test]
        fn deserialize_empty() {
            let rs: RuleSet = serde_json::from_str("{}").unwrap();
            assert_eq!(rs.len(), 0);
        }

        #[test]
        fn deserialize_required() {
            let rs: RuleSet = serde_json::from_str(r#"{"required": true}"#).unwrap();
            assert_eq!(rs.len(), 1);
            assert!(matches!(&rs[0], Rule::Required(_)));
        }

        #[test]
        fn deserialize_unknown_keys_error() {
            let res = serde_json::from_str::<RuleSet>(r#"{"required": true, "unknown": 123}"#);
            assert!(res.is_err());
        }
    }
}
