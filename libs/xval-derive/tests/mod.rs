use xval::{AsValue, derive::Value};

#[derive(Clone, Value)]
struct User {
    pub name: String,
}

#[test]
fn as_value() {
    let user = User {
        name: "Bob".to_string(),
    };

    println!("{:#?}", user.as_value().as_struct());

    assert!(user.as_value().is_struct());
    assert!(
        user.as_value()
            .as_struct()
            .field("name".into())
            .is_some_and(|v| v.as_value() == "Bob".as_value())
    )
}
