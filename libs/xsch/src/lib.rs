mod any;
mod bool;
mod context;
mod error;
mod float;
mod int;
mod number;
pub mod rule;
mod string;

pub use any::*;
pub use bool::*;
pub use context::*;
pub use error::*;
pub use float::*;
pub use int::*;
pub use number::*;
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
