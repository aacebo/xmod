use xval::ToValue;
use xval_derive::Value;

#[derive(Clone, Value)]
struct User {
    pub name: String,
    pub tags: Vec<String>,
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
