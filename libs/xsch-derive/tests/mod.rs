use xsch::{ToSchema, Validate, Validator};
use xsch_derive::Validate;
use xval::derive::Value;
use xval::ext::StructExt;

#[derive(Clone, Default, Value, Validate)]
struct WithRules {
    #[schema(required, min = 3, options = [3, 4, 5])]
    count: i32,

    #[schema(required)]
    label: String,
}

#[derive(Clone, Default, Value, Validate)]
struct Simple {
    name: String,
    age: i32,
    active: bool,
}

#[derive(Clone, Default, Value, Validate)]
struct WithOption {
    name: Option<String>,
    count: u32,
}

#[test]
fn simple_struct_produces_object_schema() {
    let schema = Simple::default().to_schema();
    assert!(schema.is_object());
}

#[test]
fn simple_struct_has_correct_field_schemas() {
    let schema = Simple::default().to_schema();
    let obj = schema.as_object().expect("expected ObjectSchema");

    assert!(
        obj.get_field("name")
            .expect("missing 'name' field")
            .is_string(),
    );
    assert!(obj.get_field("age").expect("missing 'age' field").is_int(),);
    assert!(
        obj.get_field("active")
            .expect("missing 'active' field")
            .is_bool(),
    );
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
    assert_eq!(value.as_struct().get("name").unwrap().to_value(), "alice");
    assert_eq!(value.as_struct().get("age").unwrap().to_value(), 30_i32);
    assert_eq!(value.as_struct().get("active").unwrap().to_value(), true);
}

#[test]
fn field_rules_produce_object_schema() {
    assert!(WithRules::default().to_schema().is_object());
}

#[test]
fn field_rules_validate_valid_value() {
    let s = WithRules {
        count: 4,
        label: "hi".to_string(),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn field_rules_min_rejects_below_threshold() {
    let bad = WithRules {
        count: 2,
        label: "hi".to_string(),
    };
    assert!(bad.validate().is_err());
}

#[test]
fn field_rules_options_rejects_value_not_in_list() {
    let bad = WithRules {
        count: 6,
        label: "hi".to_string(),
    };
    assert!(bad.validate().is_err());
}

#[test]
fn optional_field_accepts_null() {
    let s = WithOption {
        name: None,
        count: 5,
    };
    assert!(s.validate().is_ok());
}

#[test]
fn unsigned_int_validates_as_number() {
    let schema = WithOption::default().to_schema();
    let obj = schema.as_object().expect("expected ObjectSchema");
    assert!(
        obj.get_field("count")
            .expect("missing 'count' field")
            .is_number()
    );
}
