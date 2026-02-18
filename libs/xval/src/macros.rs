/// Declarative macro for constructing [`Value`](crate::Value) instances with JSON-like syntax.
///
/// # Examples
///
/// ```
/// use xval::valueof;
///
/// // Null
/// let v = valueof!(null);
///
/// // Primitives (uses AsValue)
/// let v = valueof!(true);
/// let v = valueof!(42_i32);
/// let v = valueof!(3.14_f64);
/// let v = valueof!("hello");
///
/// // Arrays
/// let v = valueof!([1_i32, 2_i32, 3_i32]);
/// let v = valueof!([]);
///
/// // Structs (objects with string keys)
/// let v = valueof!({ "name": "alice", "age": 30_i32 });
/// let v = valueof!({});
///
/// // Nested
/// let v = valueof!({
///     "users": [
///         { "name": "alice", "age": 30_i32 },
///         { "name": "bob", "age": 25_i32 },
///     ],
///     "count": 2_i32,
/// });
///
/// // Type coercion (returns sub-types instead of Value)
/// let n: xval::Number = valueof!(42_i32 as number);
/// let i: xval::Int = valueof!(42_i32 as int);
/// let b: xval::Bool = valueof!(true as bool);
/// let s: xval::Str = valueof!("hello" as string);
/// ```
#[macro_export]
macro_rules! valueof {
    (null) => {
        $crate::Value::Null
    };

    ({}) => {
        $crate::Value::from_struct(
            std::collections::HashMap::<$crate::Ident, $crate::Value>::new(),
        )
    };

    ({ $( $key:literal : $value:tt ),+ $(,)? }) => {{
        let mut map = std::collections::HashMap::<$crate::Ident, $crate::Value>::new();
        $(
            map.insert($crate::Ident::key($key), $crate::valueof!($value));
        )+
        $crate::Value::from_struct(map)
    }};

    ([]) => {
        $crate::Value::from_array(Vec::<$crate::Value>::new())
    };

    ([ $( $element:tt ),+ $(,)? ]) => {
        $crate::Value::from_array(vec![ $( $crate::valueof!($element) ),+ ])
    };

    ($value:tt as number) => { $crate::Number::from($value) };
    ($value:tt as int)    => { $crate::Int::from($value) };
    ($value:tt as uint)   => { $crate::UInt::from($value) };
    ($value:tt as float)  => { $crate::Float::from($value) };
    ($value:tt as bool)   => { $crate::Bool::from($value) };
    ($value:tt as string) => { $crate::Str::from($value) };
    ($value:tt as object) => { $crate::Object::from($value) };

    ($other:expr) => {
        $crate::AsValue::as_value(&$other)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn null() {
        let v = valueof!(null);
        assert!(v.is_null());
    }

    #[test]
    fn bool_true() {
        let v = valueof!(true);
        assert_eq!(v.to_bool(), true);
    }

    #[test]
    fn bool_false() {
        let v = valueof!(false);
        assert_eq!(v.to_bool(), false);
    }

    #[test]
    fn number_i32() {
        let v = valueof!(42_i32);
        assert_eq!(v.to_i32(), 42);
    }

    #[test]
    fn number_f64() {
        let v = valueof!(3.14_f64);
        assert_eq!(v.to_f64(), 3.14);
    }

    #[test]
    fn string_literal() {
        let v = valueof!("hello");
        assert_eq!(v.as_str(), "hello");
    }

    #[test]
    fn empty_array() {
        let v = valueof!([]);
        assert!(v.is_array());
        assert_eq!(v.as_array().len(), 0);
    }

    #[test]
    fn array() {
        let v = valueof!([1_i32, 2_i32, 3_i32]);
        assert!(v.is_array());
        assert_eq!(v.as_array().len(), 3);
        assert_eq!(v.as_array().index(0).unwrap().as_value().to_i32(), 1);
        assert_eq!(v.as_array().index(1).unwrap().as_value().to_i32(), 2);
        assert_eq!(v.as_array().index(2).unwrap().as_value().to_i32(), 3);
    }

    #[test]
    fn array_trailing_comma() {
        let v = valueof!([1_i32, 2_i32,]);
        assert!(v.is_array());
        assert_eq!(v.as_array().len(), 2);
    }

    #[test]
    fn single_element_array() {
        let v = valueof!([42_i32]);
        assert!(v.is_array());
        assert_eq!(v.as_array().len(), 1);
    }

    #[test]
    fn empty_struct() {
        let v = valueof!({});
        assert!(v.is_struct());
        assert_eq!(v.as_struct().len(), 0);
    }

    #[test]
    fn struct_with_fields() {
        let v = valueof!({ "a": 1_i32, "b": "hello" });
        assert!(v.is_struct());
        assert_eq!(
            v.as_struct().field("a".into()).unwrap().as_value().to_i32(),
            1
        );
        assert_eq!(
            v.as_struct().field("b".into()).unwrap().as_value().as_str(),
            "hello"
        );
    }

    #[test]
    fn struct_trailing_comma() {
        let v = valueof!({ "a": 1_i32, });
        assert!(v.is_struct());
        assert_eq!(v.as_struct().len(), 1);
    }

    #[test]
    fn nested_struct_in_array() {
        let v = valueof!([{ "name": "alice" }, { "name": "bob" }]);
        assert!(v.is_array());
        assert_eq!(v.as_array().len(), 2);
        assert_eq!(
            v.as_array()
                .index(0)
                .unwrap()
                .as_value()
                .as_struct()
                .field("name".into())
                .unwrap()
                .as_value()
                .as_str(),
            "alice"
        );
    }

    #[test]
    fn nested_array_in_struct() {
        let v = valueof!({ "items": [42_i32, 99_i32] });
        assert!(v.is_struct());
        let items = v.as_struct().field("items".into()).unwrap().as_value();
        assert!(items.is_array());
        assert_eq!(items.as_array().index(0).unwrap().as_value().to_i32(), 42);
        assert_eq!(items.as_array().index(1).unwrap().as_value().to_i32(), 99);
    }

    #[test]
    fn deeply_nested() {
        let v = valueof!({
            "users": [
                { "name": "alice", "age": 30_i32 },
                { "name": "bob", "age": 25_i32 },
            ],
            "count": 2_i32,
        });
        assert!(v.is_struct());
        assert_eq!(
            v.as_struct()
                .field("count".into())
                .unwrap()
                .as_value()
                .to_i32(),
            2
        );
        let users = v.as_struct().field("users".into()).unwrap().as_value();
        assert!(users.is_array());
        assert_eq!(users.as_array().len(), 2);
    }

    #[test]
    fn mixed_types_in_array() {
        let v = valueof!([1_i32, true, "hello"]);
        assert!(v.is_array());
        assert_eq!(v.as_array().len(), 3);
        assert_eq!(v.as_array().index(0).unwrap().as_value().to_i32(), 1);
        assert_eq!(v.as_array().index(1).unwrap().as_value().to_bool(), true);
        assert_eq!(v.as_array().index(2).unwrap().as_value().as_str(), "hello");
    }

    #[test]
    fn null_in_struct() {
        let v = valueof!({ "x": null });
        assert!(
            v.as_struct()
                .field("x".into())
                .unwrap()
                .as_value()
                .is_null()
        );
    }

    #[test]
    fn null_in_array() {
        let v = valueof!([null, 1_i32]);
        assert!(v.as_array().index(0).unwrap().as_value().is_null());
    }

    #[test]
    fn variable_interpolation() {
        let x = 42_i32;
        let v = valueof!((x));
        assert_eq!(v.to_i32(), 42);
    }

    #[test]
    fn as_number() {
        let n: crate::Number = valueof!(42_i32 as number);
        assert!(n.is_int());
        assert_eq!(n.to_i32(), 42);
    }

    #[test]
    fn as_int() {
        let i: crate::Int = valueof!(42_i32 as int);
        assert_eq!(i.to_i32(), 42);
    }

    #[test]
    fn as_uint() {
        let u: crate::UInt = valueof!(42_u32 as uint);
        assert_eq!(u.to_u32(), 42);
    }

    #[test]
    fn as_float() {
        let f: crate::Float = valueof!(3.14_f64 as float);
        assert_eq!(f.to_f64(), 3.14);
    }

    #[test]
    fn as_bool() {
        let b: crate::Bool = valueof!(true as bool);
        assert_eq!(b.to_bool(), true);
    }

    #[test]
    fn as_string() {
        let s: crate::Str = valueof!("hello" as string);
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn as_number_from_variable() {
        let x = 42_i32;
        let n: crate::Number = valueof!(x as number);
        assert_eq!(n.to_i32(), 42);
    }
}
