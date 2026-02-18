mod any;
mod array;
mod bool;
mod context;
mod error;
mod float;
mod int;
mod number;
mod phase;
pub mod rule;
mod string;

pub use any::*;
pub use array::*;
pub use bool::*;
pub use context::*;
pub use error::*;
pub use float::*;
pub use int::*;
pub use number::*;
pub use phase::*;
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
    Number(NumberSchema),
    Int(IntSchema),
    Float(FloatSchema),
    Array(ArraySchema),
}

impl Validate for Schema {
    fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
        match self {
            Self::Any(v) => v.validate(ctx),
            Self::Bool(v) => v.validate(ctx),
            Self::String(v) => v.validate(ctx),
            Self::Number(v) => v.validate(ctx),
            Self::Int(v) => v.validate(ctx),
            Self::Float(v) => v.validate(ctx),
            Self::Array(v) => v.validate(ctx),
        }
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::Any(AnySchema::default())
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

        #[test]
        fn array_dispatches() {
            let schema = Schema::Array(array());
            assert!(schema.validate(&vec![1i32, 2, 3].as_value().into()).is_ok());
            assert!(schema.validate(&42i32.as_value().into()).is_err());
        }

        #[test]
        fn array_allows_null() {
            let schema = Schema::Array(array());
            assert!(schema.validate(&xval::Value::Null.into()).is_ok());
        }

        #[test]
        fn array_required_rejects_null() {
            let schema = Schema::Array(array().required());
            assert!(schema.validate(&xval::Value::Null.into()).is_err());
        }

        #[test]
        fn array_min() {
            let schema = Schema::Array(array().min(2));
            assert!(schema.validate(&vec![1i32, 2, 3].as_value().into()).is_ok());
            assert!(schema.validate(&vec![1i32].as_value().into()).is_err());
        }

        #[test]
        fn array_max() {
            let schema = Schema::Array(array().max(2));
            assert!(schema.validate(&vec![1i32].as_value().into()).is_ok());
            assert!(
                schema
                    .validate(&vec![1i32, 2, 3].as_value().into())
                    .is_err()
            );
        }

        #[test]
        fn array_items_valid() {
            let schema = Schema::Array(array().items(string().into()));
            let value = vec!["a".to_string(), "b".to_string()].as_value();
            assert!(schema.validate(&value.into()).is_ok());
        }

        #[test]
        fn array_items_invalid() {
            let schema = Schema::Array(array().items(string().into()));
            let value = vec![1i32, 2].as_value();
            assert!(schema.validate(&value.into()).is_err());
        }

        #[test]
        fn array_items_with_inner_rules() {
            let schema = Schema::Array(array().items(int().required().into()));
            let value = vec![1i32, 2, 3].as_value();
            assert!(schema.validate(&value.into()).is_ok());
        }

        #[test]
        fn array_combined_rules() {
            let schema = Schema::Array(array().required().min(1).max(3).items(int().into()));
            assert!(schema.validate(&vec![1i32, 2].as_value().into()).is_ok());
            assert!(schema.validate(&xval::Value::Null.into()).is_err());
            assert!(
                schema
                    .validate(&Vec::<i32>::new().as_value().into())
                    .is_err()
            );
            assert!(
                schema
                    .validate(&vec![1i32, 2, 3, 4].as_value().into())
                    .is_err()
            );
        }

        #[test]
        fn array_empty_allowed() {
            let schema = Schema::Array(array());
            assert!(
                schema
                    .validate(&Vec::<i32>::new().as_value().into())
                    .is_ok()
            );
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
        fn serialize_string_empty() {
            let schema = Schema::String(string());
            let json = serde_json::to_string(&schema).unwrap();
            assert_eq!(json, r#"{"type":"string"}"#);
        }

        #[test]
        fn serialize_string_with_rules() {
            let schema = Schema::String(string().required().equals("hello"));
            let json = serde_json::to_string(&schema).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["type"], "string");
            assert_eq!(v["required"], true);
            assert_eq!(v["equals"], "hello");
        }

        #[test]
        fn deserialize_string() {
            let schema: Schema = serde_json::from_str(r#"{"type": "string"}"#).unwrap();
            assert!(matches!(schema, Schema::String(_)));
        }

        #[test]
        fn roundtrip_string_with_rules() {
            let json = r#"{"type":"string","required":true,"equals":"hello"}"#;
            let schema: Schema = serde_json::from_str(json).unwrap();
            assert!(matches!(schema, Schema::String(_)));

            let reserialized = serde_json::to_string(&schema).unwrap();
            let v1: serde_json::Value = serde_json::from_str(json).unwrap();
            let v2: serde_json::Value = serde_json::from_str(&reserialized).unwrap();
            assert_eq!(v1, v2);
        }

        #[test]
        fn serialize_number_empty() {
            let schema = Schema::Number(number());
            let json = serde_json::to_string(&schema).unwrap();
            assert_eq!(json, r#"{"type":"number"}"#);
        }

        #[test]
        fn serialize_number_with_rules() {
            let schema = Schema::Number(number().required().equals(xval::Number::from_i32(42)));
            let json = serde_json::to_string(&schema).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["type"], "number");
            assert_eq!(v["required"], true);
            assert_eq!(v["equals"], 42);
        }

        #[test]
        fn deserialize_number() {
            let schema: Schema = serde_json::from_str(r#"{"type": "number"}"#).unwrap();
            assert!(matches!(schema, Schema::Number(_)));
        }

        #[test]
        fn roundtrip_number_with_rules() {
            let json = r#"{"type":"number","required":true,"options":[1,2,3]}"#;
            let schema: Schema = serde_json::from_str(json).unwrap();
            assert!(matches!(schema, Schema::Number(_)));

            let reserialized = serde_json::to_string(&schema).unwrap();
            let v1: serde_json::Value = serde_json::from_str(json).unwrap();
            let v2: serde_json::Value = serde_json::from_str(&reserialized).unwrap();
            assert_eq!(v1, v2);
        }

        #[test]
        fn serialize_int_empty() {
            let schema = Schema::Int(int());
            let json = serde_json::to_string(&schema).unwrap();
            assert_eq!(json, r#"{"type":"int"}"#);
        }

        #[test]
        fn serialize_int_with_rules() {
            let schema = Schema::Int(int().required().equals(xval::Int::from_i32(10)));
            let json = serde_json::to_string(&schema).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["type"], "int");
            assert_eq!(v["required"], true);
            assert_eq!(v["equals"], 10);
        }

        #[test]
        fn deserialize_int() {
            let schema: Schema = serde_json::from_str(r#"{"type": "int"}"#).unwrap();
            assert!(matches!(schema, Schema::Int(_)));
        }

        #[test]
        fn roundtrip_int_with_rules() {
            let json = r#"{"type":"int","required":true,"options":[1,2,3]}"#;
            let schema: Schema = serde_json::from_str(json).unwrap();
            assert!(matches!(schema, Schema::Int(_)));

            let reserialized = serde_json::to_string(&schema).unwrap();
            let v1: serde_json::Value = serde_json::from_str(json).unwrap();
            let v2: serde_json::Value = serde_json::from_str(&reserialized).unwrap();
            assert_eq!(v1, v2);
        }

        #[test]
        fn serialize_float_empty() {
            let schema = Schema::Float(float());
            let json = serde_json::to_string(&schema).unwrap();
            assert_eq!(json, r#"{"type":"float"}"#);
        }

        #[test]
        fn serialize_float_with_rules() {
            let schema = Schema::Float(float().required().equals(xval::Float::from_f64(3.14)));
            let json = serde_json::to_string(&schema).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["type"], "float");
            assert_eq!(v["required"], true);
            assert_eq!(v["equals"], 3.14);
        }

        #[test]
        fn deserialize_float() {
            let schema: Schema = serde_json::from_str(r#"{"type": "float"}"#).unwrap();
            assert!(matches!(schema, Schema::Float(_)));
        }

        #[test]
        fn roundtrip_float_with_rules() {
            let json = r#"{"type":"float","required":true,"options":[1.0,2.5,3.14]}"#;
            let schema: Schema = serde_json::from_str(json).unwrap();
            assert!(matches!(schema, Schema::Float(_)));

            let reserialized = serde_json::to_string(&schema).unwrap();
            let v1: serde_json::Value = serde_json::from_str(json).unwrap();
            let v2: serde_json::Value = serde_json::from_str(&reserialized).unwrap();
            assert_eq!(v1, v2);
        }

        #[test]
        fn serialize_array_empty() {
            let schema = Schema::Array(array());
            let json = serde_json::to_string(&schema).unwrap();
            assert_eq!(json, r#"{"type":"array"}"#);
        }

        #[test]
        fn serialize_array_with_rules() {
            let schema = Schema::Array(array().required().min(1).max(10));
            let json = serde_json::to_string(&schema).unwrap();
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(v["type"], "array");
            assert_eq!(v["required"], true);
            assert_eq!(v["min"], 1);
            assert_eq!(v["max"], 10);
        }

        #[test]
        fn deserialize_array() {
            let schema: Schema = serde_json::from_str(r#"{"type": "array"}"#).unwrap();
            assert!(matches!(schema, Schema::Array(_)));
        }

        #[test]
        fn roundtrip_array_with_rules() {
            let json = r#"{"type":"array","required":true,"min":1,"max":10}"#;
            let schema: Schema = serde_json::from_str(json).unwrap();
            assert!(matches!(schema, Schema::Array(_)));

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
