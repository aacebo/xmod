use xsch::{AsSchema, Schema, Validate, derive::Validate};
use xval::AsValue;

#[derive(Clone, Validate)]
struct Simple {
    name: xsch::StringSchema,
    age: xsch::IntSchema,
    active: xsch::BoolSchema,
}

#[test]
fn simple_struct_produces_object_schema() {
    let s = Simple {
        name: xsch::string().required(),
        age: xsch::int().min(0),
        active: xsch::bool(),
    };

    let schema = s.as_schema();
    assert!(matches!(schema, Schema::Object(_)));
}

#[test]
fn simple_struct_validates_matching_value() {
    let s = Simple {
        name: xsch::string(),
        age: xsch::int(),
        active: xsch::bool(),
    };

    let schema = s.as_schema();

    let mut map = std::collections::HashMap::new();
    map.insert(xval::Ident::key("name"), xval::valueof!("alice"));
    map.insert(xval::Ident::key("age"), xval::valueof!(30_i32));
    map.insert(xval::Ident::key("active"), xval::valueof!(true));

    assert!(schema.validate(&map.as_value().into()).is_ok());
}

#[test]
fn simple_struct_rejects_wrong_type() {
    let s = Simple {
        name: xsch::string(),
        age: xsch::int(),
        active: xsch::bool(),
    };

    let schema = s.as_schema();

    let mut map = std::collections::HashMap::new();
    map.insert(xval::Ident::key("name"), xval::valueof!(123_i32));
    map.insert(xval::Ident::key("age"), xval::valueof!(30_i32));
    map.insert(xval::Ident::key("active"), xval::valueof!(true));

    assert!(schema.validate(&map.as_value().into()).is_err());
}

#[test]
fn simple_struct_enforces_rules() {
    let s = Simple {
        name: xsch::string().required(),
        age: xsch::int().min(0).max(150),
        active: xsch::bool(),
    };

    let schema = s.as_schema();

    // missing required name (null)
    let mut map = std::collections::HashMap::new();
    map.insert(xval::Ident::key("name"), xval::valueof!(null));
    map.insert(xval::Ident::key("age"), xval::valueof!(30_i32));
    map.insert(xval::Ident::key("active"), xval::valueof!(true));

    assert!(schema.validate(&map.as_value().into()).is_err());
}

#[derive(Clone, xsch_derive::Validate)]
struct Person {
    name: xsch::StringSchema,
    address: xsch::ObjectSchema,
}

#[test]
fn nested_struct_validates() {
    let p = Person {
        name: xsch::string(),
        address: xsch::object().field("street", xsch::string().into()),
    };

    let schema = p.as_schema();

    let mut inner = std::collections::HashMap::new();
    inner.insert(xval::Ident::key("street"), xval::valueof!("123 Main St"));

    let mut outer = std::collections::HashMap::new();
    outer.insert(xval::Ident::key("name"), xval::valueof!("alice"));
    outer.insert(xval::Ident::key("address"), inner.as_value());

    assert!(schema.validate(&outer.as_value().into()).is_ok());
}

#[test]
fn nested_struct_rejects_invalid_inner() {
    let p = Person {
        name: xsch::string(),
        address: xsch::object().field("street", xsch::string().into()),
    };

    let schema = p.as_schema();

    let mut inner = std::collections::HashMap::new();
    inner.insert(xval::Ident::key("street"), xval::valueof!(42_i32));

    let mut outer = std::collections::HashMap::new();
    outer.insert(xval::Ident::key("name"), xval::valueof!("alice"));
    outer.insert(xval::Ident::key("address"), inner.as_value());

    assert!(schema.validate(&outer.as_value().into()).is_err());
}
