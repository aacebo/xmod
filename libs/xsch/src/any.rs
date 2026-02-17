use crate::{Context, ValidError, Validate, rules::Rule};

#[derive(Debug, Default, Clone)]
pub struct AnySchema(Vec<Rule>);

impl AnySchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn exists(&self, name: &str) -> bool {
        self.0.iter().find(|r| r.name() == name).is_some()
    }

    pub fn get(&self, name: &str) -> Option<&Rule> {
        self.0.iter().find(|r| r.name() == name)
    }

    pub fn register(&mut self, rule: Rule) -> &mut Self {
        self.0.push(rule);
        self
    }

    pub fn rule(mut self, rule: Rule) -> Self {
        self.register(rule);
        self
    }

    pub fn validate(&self, value: &xval::Value) -> Result<xval::Value, ValidError> {
        Validate::validate(
            self,
            &Context {
                rule: "type::any".to_string(),
                path: xpath::Path::default(),
                value: value.clone(),
            },
        )
    }
}

impl std::fmt::Display for AnySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Validate for AnySchema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        let mut next = ctx.clone();
        let mut error = ValidError::new(&ctx.rule, ctx.path.clone()).build();

        for rule in &self.0 {
            next.rule = rule.name().to_string();
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
impl serde::Serialize for AnySchema {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = s.serialize_map(Some(self.0.len()))?;

        for rule in &self.0 {
            map.serialize_entry(rule.name(), rule)?;
        }

        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AnySchema {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AnySchemaVisitor;

        impl<'de> serde::de::Visitor<'de> for AnySchemaVisitor {
            type Value = AnySchema;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a map of schema rules")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut rules = Vec::new();

                while let Some(key) = map.next_key::<String>()? {
                    if let Some(rule) = Rule::deserialize_by_name(&key, &mut map)? {
                        rules.push(rule);
                    } else {
                        let _ = map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }

                Ok(AnySchema(rules))
            }
        }

        d.deserialize_map(AnySchemaVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xval::AsValue;

    #[test]
    fn required_passes_when_non_null() {
        let schema = AnySchema::new().required();
        let result = schema.validate(&"hello".as_value());
        assert!(result.is_ok());
    }

    #[test]
    fn required_fails_when_null() {
        let schema = AnySchema::new().required();
        let result = schema.validate(&xval::Value::Null);
        let err = result.unwrap_err();
        assert_eq!(err.rule, "type::any");
        assert_eq!(err.errors.len(), 1);
        assert_eq!(err.errors[0].rule, "required");
    }

    #[test]
    fn one_of_passes_when_value_matches() {
        let options = vec!["a".as_value(), "b".as_value(), "c".as_value()];
        let schema = AnySchema::new().options(&options);
        let result = schema.validate(&"b".as_value());
        assert!(result.is_ok());
    }

    #[test]
    fn one_of_fails_when_value_not_in_options() {
        let options = vec!["a".as_value(), "b".as_value(), "c".as_value()];
        let schema = AnySchema::new().options(&options);
        let result = schema.validate(&"d".as_value());
        let err = result.unwrap_err();
        assert_eq!(err.rule, "type::any");
        assert_eq!(err.errors.len(), 1);
        assert_eq!(err.errors[0].rule, "one_of");
    }

    #[test]
    fn required_and_one_of_passes() {
        let options = vec!["a".as_value(), "b".as_value()];
        let schema = AnySchema::new().required().options(&options);
        let result = schema.validate(&"a".as_value());
        assert!(result.is_ok());
    }

    #[test]
    fn required_and_one_of_fails_on_null() {
        let options = vec!["a".as_value(), "b".as_value()];
        let schema = AnySchema::new().required().options(&options);
        let result = schema.validate(&xval::Value::Null);
        let err = result.unwrap_err();
        assert_eq!(err.rule, "type::any");
        // Vec preserves insertion order: "required" before "one_of"
        assert_eq!(err.errors.len(), 2);
        assert_eq!(err.errors[0].rule, "required");
        assert_eq!(err.errors[1].rule, "one_of");
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use crate::Schema;

        #[test]
        fn serialize_required() {
            let schema = Schema::Any(AnySchema::new().required());
            let json = serde_json::to_value(&schema).unwrap();
            assert_eq!(json, serde_json::json!({"type": "any", "required": true}));
        }

        #[test]
        fn serialize_one_of() {
            let options = vec!["a".as_value(), "b".as_value()];
            let schema = Schema::Any(AnySchema::new().options(&options));
            let json = serde_json::to_value(&schema).unwrap();
            assert_eq!(
                json,
                serde_json::json!({"type": "any", "one_of": ["a", "b"]})
            );
        }

        #[test]
        fn serialize_required_and_one_of() {
            let options = vec!["a".as_value(), "b".as_value()];
            let schema = Schema::Any(AnySchema::new().required().options(&options));
            let json = serde_json::to_value(&schema).unwrap();
            assert_eq!(
                json,
                serde_json::json!({
                    "type": "any",
                    "required": true,
                    "one_of": ["a", "b"]
                })
            );
        }

        #[test]
        fn deserialize_required() {
            let schema: Schema =
                serde_json::from_str(r#"{"type": "any", "required": true}"#).unwrap();

            match &schema {
                Schema::Any(s) => {
                    assert_eq!(s.len(), 1);
                    assert!(s.exists("required"));
                }
            }
        }

        #[test]
        fn deserialize_required_and_one_of() {
            let schema: Schema =
                serde_json::from_str(r#"{"type": "any", "required": true, "one_of": ["a", "b"]}"#)
                    .unwrap();

            match &schema {
                Schema::Any(s) => {
                    assert_eq!(s.len(), 2);
                    assert!(s.exists("required"));
                    assert!(s.exists("one_of"));
                }
            }
        }

        #[test]
        fn roundtrip() {
            let options = vec!["x".as_value(), "y".as_value()];
            let original = Schema::Any(AnySchema::new().required().options(&options));
            let json = serde_json::to_string(&original).unwrap();
            let restored: Schema = serde_json::from_str(&json).unwrap();

            match &restored {
                Schema::Any(s) => {
                    assert_eq!(s.len(), 2);
                    assert!(s.exists("required"));
                    assert!(s.exists("one_of"));
                }
            }
        }
    }
}
