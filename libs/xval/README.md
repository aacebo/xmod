# xval

Dynamically-typed values for Rust. Represent any data — nulls, booleans, numbers at every bit width, strings, and nested objects — as a single `Value` type with full runtime inspection.

## Creating Values

The `valueof!` macro gives you JSON-like syntax:

```rust
use xval::valueof;

let null = valueof!(null);
let b    = valueof!(true);
let n    = valueof!(42_i32);
let f    = valueof!(3.14_f64);
let s    = valueof!("hello");

let arr = valueof!([1_i32, 2_i32, 3_i32]);
let obj = valueof!({ "name": "alice", "age": 30_i32 });

let nested = valueof!({
    "users": [
        { "name": "alice", "active": true },
        { "name": "bob", "active": false },
    ],
    "count": 2_i32,
});
```

Variables and expressions work when wrapped in parentheses:

```rust
let x = 42_i32;
let v = valueof!((x));
let computed = valueof!((x + 1));
```

### Extracting Sub-Types

Use `as` to get a typed value instead of `Value`:

```rust
let n: xval::Number = valueof!(42_i32 as number);
let i: xval::Int    = valueof!(42_i32 as int);
let s: xval::Str    = valueof!("hello" as string);
```

## Reading Values

```rust
let v = valueof!({ "name": "alice", "scores": [90_i32, 85_i32] });

assert!(v.is_struct());
assert_eq!(v.as_struct().field("name".into()).unwrap().to_value().as_str(), "alice");

let scores = v.as_struct().field("scores".into()).unwrap().to_value();
assert_eq!(scores.as_array().len(), 2);
assert_eq!(scores.as_array().index(0).unwrap().to_value().to_i32(), 90);
```

### Type Checking

```rust
let v = valueof!(42_i32);

assert!(v.is_number());
assert!(v.is_int());
assert!(!v.is_string());
```

### Navigating with Paths

Traverse nested structures using `xpath::Path`:

```rust
let data = valueof!({
    "users": [{ "name": "alice" }],
});

let path = xpath::Path::parse("users/0/name").unwrap();
let name = data.get(&path).unwrap();
assert_eq!(name.as_str(), "alice");
```

## Converting Your Types

Any type can produce a `Value` by implementing `ToValue`:

```rust
use xval::ToValue;

let v = 42_i32.to_value();
let v = "hello".to_value();
let v = vec![1_i32, 2, 3].to_value();
```

## Derive Macro

Enable the `derive` feature to auto-implement `ToValue` for custom types:

```toml
[dependencies]
xval = { version = "0.0.0", features = ["derive"] }
```

### Structs

```rust
#[derive(Clone, xval::derive::Value)]
struct User {
    name: String,
    age: i32,
}

let user = User { name: "alice".into(), age: 30 };
let v = user.to_value();

assert!(v.is_struct());
assert_eq!(v.as_struct().field("name".into()).unwrap().to_value().as_str(), "alice");
```

### Tuple Structs

```rust
#[derive(Clone, xval::derive::Value)]
struct Point(i32, i32);

let v = Point(1, 2).to_value();
assert!(v.is_tuple());
```

### Enums

Each variant maps to its natural shape:

```rust
#[derive(Clone, xval::derive::Value)]
enum Message {
    Disconnect,                          // -> null
    Text(String),                        // -> tuple
    Chat { user: String, text: String }, // -> struct
}

assert!(Message::Disconnect.to_value().is_null());
assert!(Message::Text("hi".into()).to_value().is_tuple());
```

### Unit Structs

```rust
#[derive(Clone, xval::derive::Value)]
struct Empty;

assert!(Empty.to_value().is_null());
```

## Value Hierarchy

```
Value
  Null
  Bool
  Number
    Int   (i8, i16, i32, i64, i128)
    UInt  (u8, u16, u32, u64, u128)
    Float (f32, f64)
  String
  Object
    Struct  (named fields)
    Array   (indexed elements)
    Tuple   (positional elements)
```

## Features

| Feature | Description |
|---------|-------------|
| `derive` | `#[derive(Value)]` for custom types via `xval-derive` |
| `serde` | `Serialize`/`Deserialize` for all value types |
