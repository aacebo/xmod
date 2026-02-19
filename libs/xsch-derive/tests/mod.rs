use xsch::{AsSchema, Validate, Validator};
use xsch_derive::Validate;
use xval::derive::Value;

#[derive(Clone, Default, Value, Validate)]
struct WithRules {
    #[field(required, min = 3, options = [3, 4, 5])]
    count: i32,

    #[field(required)]
    label: String,
}

#[derive(Clone, Default, Value, Validate)]
struct Simple {
    name: String,
    age: i32,
    active: bool,
}

// ── Simple (no #[field] annotations) ────────────────────────────────────────

#[test]
fn simple_struct_produces_object_schema() {
    let schema = Simple::default().as_schema();
    assert!(schema.is_object());
}

#[test]
fn simple_struct_has_correct_field_schemas() {
    let schema = Simple::default().as_schema();
    let obj = schema.as_object().expect("expected ObjectSchema");

    assert!(
        obj.get_field("name")
            .expect("missing 'name' field")
            .is_string(),
        "expected 'name' to have StringSchema"
    );
    assert!(
        obj.get_field("age").expect("missing 'age' field").is_int(),
        "expected 'age' to have IntSchema"
    );
    assert!(
        obj.get_field("active")
            .expect("missing 'active' field")
            .is_bool(),
        "expected 'active' to have BoolSchema"
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
    assert_eq!(
        value.as_struct().field("name".into()).unwrap().to_value(),
        "alice"
    );
    assert_eq!(
        value.as_struct().field("age".into()).unwrap().to_value(),
        30_i32
    );
    assert_eq!(
        value.as_struct().field("active".into()).unwrap().to_value(),
        true
    );
}

// ── WithRules (#[field] annotations) ────────────────────────────────────────

#[test]
fn field_rules_produce_object_schema() {
    assert!(WithRules::default().as_schema().is_object());
}

#[test]
fn field_rules_has_correct_field_schemas() {
    let schema = WithRules::default().as_schema();
    let obj = schema.as_object().expect("expected ObjectSchema");

    assert!(
        obj.get_field("count")
            .expect("missing 'count' field")
            .is_int(),
        "expected 'count' to have IntSchema"
    );
    assert!(
        obj.get_field("label")
            .expect("missing 'label' field")
            .is_string(),
        "expected 'label' to have StringSchema"
    );
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
