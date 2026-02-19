use xval::AsValue;
use xval_derive::Value;

#[derive(Clone, Value)]
enum Message {
    Disconnect,
    Text(String),
    Pair(i32, bool),
    Chat { user: String, text: String },
}

#[test]
fn enum_unit_variant() {
    let v = Message::Disconnect.as_value();
    assert!(v.is_null());
}

#[test]
fn enum_single_tuple_variant() {
    let v = Message::Text("hello".into()).as_value();
    assert!(v.is_tuple());
    assert_eq!(v.as_tuple().len(), 1);
    assert_eq!(v.as_tuple().index(0).unwrap().as_value().as_str(), "hello");
}

#[test]
fn enum_multi_tuple_variant() {
    let v = Message::Pair(10, true).as_value();
    assert!(v.is_tuple());

    let t = v.as_tuple();
    assert_eq!(t.len(), 2);
    assert_eq!(t.index(0).unwrap().as_value().to_i32(), 10);
    assert_eq!(t.index(1).unwrap().as_value().to_bool(), true);
}

#[test]
fn enum_named_variant() {
    let v = Message::Chat {
        user: "alice".into(),
        text: "hi".into(),
    }
    .as_value();

    assert!(v.is_struct());

    let s = v.as_struct();
    assert_eq!(s.len(), 2);
    assert_eq!(s.field("user".into()).unwrap().as_value().as_str(), "alice");
    assert_eq!(s.field("text".into()).unwrap().as_value().as_str(), "hi");
}
