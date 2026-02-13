use xval::{AsValue, derive::Value};

#[derive(Clone, Value)]
struct User {
    pub name: String,
    pub tags: Vec<String>,
}

#[test]
fn as_value() {
    let user = User {
        name: "Bob".to_string(),
        tags: vec!["a".into(), "b".into()],
    };

    assert!(user.as_value().is_struct());
    assert!(
        user.as_value()
            .as_struct()
            .field("tags".into())
            .is_some_and(|v| { v.as_value().is_array() && v.as_value().as_object().len() == 2 })
    );

    assert!(
        user.as_value()
            .as_struct()
            .field("name".into())
            .is_some_and(|v| v.as_value() == "Bob".as_value())
    );
}
