use xval::ToValue;
use xval_derive::Value;

#[derive(Value)]
struct Wrapper<T: Clone + ToValue + Send + Sync + 'static> {
    value: T,
}

#[derive(Value)]
struct Pair<A: Clone + ToValue + Send + Sync + 'static, B: Clone + ToValue + Send + Sync + 'static>
{
    first: A,
    second: B,
}

#[derive(Value)]
struct Container<T>
where
    T: Clone + ToValue + Send + Sync + 'static,
{
    items: Vec<T>,
}

#[derive(Value)]
enum Maybe<T: Clone + ToValue> {
    Nothing,
    Just(T),
    Wrapped { value: T },
}

#[test]
fn generic_struct_single_param() {
    let w = Wrapper { value: 42i32 };
    assert!(w.to_value().is_struct());
    assert_eq!(
        w.to_value()
            .as_struct()
            .field("value".into())
            .unwrap()
            .to_value()
            .to_i32(),
        42
    );
}

#[test]
fn generic_struct_multiple_params() {
    let p = Pair {
        first: "hello".to_string(),
        second: true,
    };
    assert!(p.to_value().is_struct());
    let v = p.to_value();
    let s = v.as_struct();
    assert_eq!(
        s.field("first".into()).unwrap().to_value().as_str(),
        "hello"
    );
    assert_eq!(s.field("second".into()).unwrap().to_value().to_bool(), true);
}

#[test]
fn generic_struct_where_clause() {
    let c = Container {
        items: vec![1i32, 2, 3],
    };
    assert!(c.to_value().is_struct());
    let v = c.to_value();
    let s = v.as_struct();
    assert!(
        s.field("items".into())
            .is_some_and(|v| v.to_value().is_array())
    );
    assert_eq!(
        s.field("items".into())
            .unwrap()
            .to_value()
            .as_object()
            .len(),
        3
    );
}

#[test]
fn generic_enum_unit_variant() {
    let v = Maybe::<i32>::Nothing.to_value();
    assert!(v.is_null());
}

#[test]
fn generic_enum_tuple_variant() {
    let v = Maybe::Just(42i32).to_value();
    assert!(v.is_tuple());
    assert_eq!(v.as_tuple().index(0).unwrap().to_value().to_i32(), 42);
}

#[test]
fn generic_enum_named_variant() {
    let v = Maybe::Wrapped {
        value: "hi".to_string(),
    }
    .to_value();
    assert!(v.is_struct());
    let s = v.as_struct();
    assert_eq!(s.field("value".into()).unwrap().to_value().as_str(), "hi");
}
