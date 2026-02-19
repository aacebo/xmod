use xsch::{AsSchema, Schema, Validate, Validator, derive::Validate};
use xval::derive::Value;

#[derive(Clone, Default, Value, Validate)]
struct Simple {
    name: String,
    age: i32,
    active: bool,
}

#[test]
fn simple_struct_produces_object_schema() {
    let s = Simple {
        name: "Alex".to_string(),
        age: 32,
        active: true,
    };

    let schema = s.as_schema();
    assert!(matches!(schema, Schema::Object(_)));
}

#[test]
fn simple_struct_validates_matching_value() {
    let s = Simple {
        name: "alice".to_string(),
        age: 30,
        active: true,
    };

    let value = s.validate().unwrap();
    
    assert!(value.is_struct());
    assert_eq!(value.as_struct().field("name".into()).unwrap().as_value(), "alice");
    assert_eq!(value.as_struct().field("age".into()).unwrap().as_value(), 30_i32);
    assert_eq!(value.as_struct().field("active".into()).unwrap().as_value(), true);
}
