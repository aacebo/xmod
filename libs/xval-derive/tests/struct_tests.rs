use xval::ToValue;
use xval_derive::Value;

#[derive(Value)]
struct Pair(i32, bool);

#[derive(Value)]
struct User {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Value)]
struct WithTuple {
    pub pair: Pair,
    pub name: String,
}

#[test]
fn struct_to_value() {
    let user = User {
        name: "Bob".to_string(),
        tags: vec!["a".into(), "b".into()],
    };

    assert!(user.to_value().is_struct());
    assert!(
        user.to_value()
            .as_struct()
            .field("tags".into())
            .is_some_and(|v| { v.to_value().is_array() && v.to_value().as_object().len() == 2 })
    );

    assert!(
        user.to_value()
            .as_struct()
            .field("name".into())
            .is_some_and(|v| v.to_value() == "Bob".to_value())
    );
}

#[test]
fn struct_with_tuple_field() {
    let v = WithTuple {
        pair: Pair(42, true),
        name: "test".into(),
    }
    .to_value();

    assert!(v.is_struct());

    let s = v.as_struct();
    assert_eq!(s.len(), 2);

    let pair = s.field("pair".into()).unwrap().to_value();
    assert!(pair.is_tuple());
    assert_eq!(pair.as_tuple().len(), 2);
    assert_eq!(pair.as_tuple().index(0).unwrap().to_value().to_i32(), 42);
    assert_eq!(pair.as_tuple().index(1).unwrap().to_value().to_bool(), true);

    let name = s.field("name".into()).unwrap().to_value();
    assert!(name.is_string());
    assert_eq!(name.as_str(), "test");
}
