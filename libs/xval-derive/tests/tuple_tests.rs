use xval::ToValue;
use xval_derive::Value;

#[derive(Value)]
struct Single(i32);

#[derive(Value)]
struct Pair(i32, bool);

#[derive(Value)]
struct Unit;

#[test]
fn single_is_tuple() {
    let v = Single(42).to_value();
    assert!(v.is_tuple());
    assert_eq!(v.as_tuple().len(), 1);
    assert_eq!(v.as_tuple().index(0).unwrap().to_value().to_i32(), 42);
}

#[test]
fn pair_is_tuple() {
    let v = Pair(10, true).to_value();
    assert!(v.is_tuple());

    let t = v.as_tuple();
    assert_eq!(t.len(), 2);
    assert_eq!(t.index(0).unwrap().to_value().to_i32(), 10);
    assert_eq!(t.index(1).unwrap().to_value().to_bool(), true);
}

#[test]
fn pair_items() {
    let v = Pair(5, false).to_value();
    let items: Vec<_> = v.as_tuple().items().collect();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].to_value().to_i32(), 5);
    assert_eq!(items[1].to_value().to_bool(), false);
}

#[test]
fn pair_index_out_of_bounds() {
    let v = Pair(1, true).to_value();
    assert!(v.as_tuple().index(2).is_none());
}

#[test]
fn unit_is_null() {
    let v = Unit.to_value();
    assert!(v.is_null());
}
