mod any;
mod bool;
mod context;
mod error;
pub mod rule;
mod string;

pub use any::*;
pub use bool::*;
pub use context::*;
pub use error::*;
pub use rule::*;
pub use string::*;

pub trait Validate {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError>;
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum Schema {
    Any(AnySchema),
    Bool(BoolSchema),
    String(StringSchema),
}

impl Validate for Schema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::Any(v) => v.validate(ctx),
            Self::Bool(v) => v.validate(ctx),
            Self::String(v) => v.validate(ctx),
        }
    }
}

#[cfg(test)]
mod tests {
    use xval::AsValue;

    use super::*;

    mod validate {
        use super::*;

        #[test]
        fn any_dispatches() {
            let schema = Schema::Any(any());
            assert!(schema.validate(&"anything".as_value().into()).is_ok());
        }

        #[test]
        fn bool_dispatches() {
            let schema = Schema::Bool(bool());
            assert!(schema.validate(&true.as_value().into()).is_ok());
            assert!(schema.validate(&42i32.as_value().into()).is_err());
        }

        #[test]
        fn bool_with_rules_dispatches() {
            let schema = Schema::Bool(bool().required().equals(true));
            assert!(schema.validate(&true.as_value().into()).is_ok());
            assert!(schema.validate(&false.as_value().into()).is_err());
            assert!(schema.validate(&xval::Value::Null.into()).is_err());
        }
    }

    #[cfg(feature = "serde")]
    mod serde {
        use xval::AsValue;

        use crate::*;

        #[test]
        fn serialize_any_empty() {
            let schema = Schema::Any(any());
            let json = serde_json::to_string(&schema).unwrap();
            assert_eq!(json, r#"{"type":"any"}"#);
        }

        #[test]
        fn serialize_bool_empty() {
            let schema = Schema::Bool(bool());
            let json = serde_json::to_string(&schema).unwrap();
            assert_eq!(json, r#"{"type":"bool"}"#);
        }

        #[test]
        fn serialize_any_with_rules() {
            let schema = Schema::Any(any().required().options(&[
                1i32.as_value(),
                "test".as_value(),
                true.as_value(),
            ]));
            let json = serde_json::to_string(&schema).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["type"], "any");
            assert_eq!(v["required"], true);
            assert_eq!(v["options"], serde_json::json!([1, "test", true]));
        }

        #[test]
        fn deserialize_any() {
            let schema: Schema = serde_json::from_str(r#"{"type": "any"}"#).unwrap();
            assert!(matches!(schema, Schema::Any(_)));
        }

        #[test]
        fn deserialize_bool() {
            let schema: Schema = serde_json::from_str(r#"{"type": "bool"}"#).unwrap();
            assert!(matches!(schema, Schema::Bool(_)));
        }

        #[test]
        fn roundtrip_any_with_rules() {
            let json = r#"{"type":"any","required":true,"options":[1,"test",true]}"#;
            let schema: Schema = serde_json::from_str(json).unwrap();
            assert!(matches!(schema, Schema::Any(_)));

            let reserialized = serde_json::to_string(&schema).unwrap();
            let v1: serde_json::Value = serde_json::from_str(json).unwrap();
            let v2: serde_json::Value = serde_json::from_str(&reserialized).unwrap();
            assert_eq!(v1, v2);
        }

        #[test]
        fn deserialize_missing_type_errors() {
            let result = serde_json::from_str::<Schema>(r#"{"required": true}"#);
            assert!(result.is_err());
        }

        #[test]
        fn deserialize_unknown_type_errors() {
            let result = serde_json::from_str::<Schema>(r#"{"type": "nonexistent"}"#);
            assert!(result.is_err());
        }
    }
}
