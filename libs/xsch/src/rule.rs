use crate::{Context, Equals, Options, Required, ValidError, Validate};

#[derive(Debug, Clone)]
pub enum Rule {
    Equals(Equals),
    Required(Required),
    Options(Options),
}

impl Rule {
    pub fn key(&self) -> &str {
        match self {
            Self::Equals(_) => Equals::KEY,
            Self::Options(_) => Options::KEY,
            Self::Required(_) => Required::KEY,
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
            _ => {
                let _ = map.next_value::<serde::de::IgnoredAny>()?;
                Ok(None)
            }
        }
    }
}

impl Validate for Rule {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::Equals(v) => v.validate(ctx),
            Self::Options(v) => v.validate(ctx),
            Self::Required(v) => v.validate(ctx),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RuleSet(Vec<Rule>);

impl RuleSet {
    pub fn add(mut self, rule: Rule) -> Self {
        self.0.push(rule);
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
        let mut error = ValidError::new(ctx.path.clone()).build();

        for rule in &self.0 {
            next.rule = Some(rule.key().to_string());
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
                let mut rules = Vec::new();

                while let Some(key) = map.next_key::<String>()? {
                    if let Some(rule) = Rule::deserialize_entry(&key, &mut map)? {
                        rules.push(rule);
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
        fn deserialize_unknown_keys_ignored() {
            let rs: RuleSet =
                serde_json::from_str(r#"{"required": true, "unknown": 123}"#).unwrap();
            assert_eq!(rs.len(), 1);
        }
    }
}
