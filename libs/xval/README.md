# xval

A dynamically-typed value system for Rust. `xval` provides a `Value` enum that can represent nulls, booleans, numbers (signed, unsigned, float at various bit widths), strings, and composite objects (structs, arrays, tuples) with full runtime reflection.

## Value Types

```
Value
  Null
  Bool(Bool)
  Number(Number)
    Int(I8 | I16 | I32 | I64 | I128)
    UInt(U8 | U16 | U32 | U64 | U128)
    Float(F32 | F64)
  String(Str)
  Object(Object)
    Struct(Arc<dyn Struct>)
    Array(Arc<dyn Array>)
    Tuple(Arc<dyn Tuple>)
```

## The `valueof!` Macro

The preferred way to construct `Value` instances. Uses JSON-like syntax:

```rust
use xval::valueof;

// Primitives
let null = valueof!(null);
let b = valueof!(true);
let n = valueof!(42_i32);
let f = valueof!(3.14_f64);
let s = valueof!("hello");

// Arrays
let arr = valueof!([1_i32, 2_i32, 3_i32]);

// Structs (string-keyed objects)
let obj = valueof!({ "name": "alice", "age": 30_i32 });

// Nested structures
let data = valueof!({
    "users": [
        { "name": "alice", "active": true },
        { "name": "bob", "active": false },
    ],
    "count": 2_i32,
});

// Variables and expressions (wrap in parentheses)
let x = 42_i32;
let v = valueof!((x));
let computed = valueof!((x + 1));
```

The macro works with any type that implements `AsValue`, including user-defined types using `#[derive(Value)]`.

### Type Coercion with `as`

Use `as` to return a sub-type instead of `Value`:

```rust
use xval::valueof;

let n: xval::Number = valueof!(42_i32 as number);
let i: xval::Int    = valueof!(42_i32 as int);
let u: xval::UInt   = valueof!(42_u32 as uint);
let f: xval::Float  = valueof!(3.14_f64 as float);
let b: xval::Bool   = valueof!(true as bool);
let s: xval::Str    = valueof!("hello" as string);

// Also works with variables
let x = 10_i64;
let n: xval::Number = valueof!(x as number);
```

## The `AsValue` Trait

The core conversion trait. Implemented for all primitive types, collections, and smart pointers:

```rust
use xval::AsValue;

let v = 42_i32.as_value();        // Value::Number(...)
let v = "hello".as_value();       // Value::String(...)
let v = vec![1, 2, 3].as_value(); // Value::Object(Array(...))
```

## Derive Macro

Enable the `derive` feature to auto-implement `AsValue` for your types:

```toml
[dependencies]
xval = { version = "0.0.0", features = ["derive"] }
```

```rust
#[derive(Clone, xval::derive::Value)]
struct User {
    name: String,
    age: i32,
}

let user = User { name: "alice".into(), age: 30 };
let v = user.as_value(); // Value::Object(Struct(...))
```

## Features

| Feature | Description |
|---------|-------------|
| `derive` | Enables `#[derive(Value)]` via `xval-derive` |
| `serde` | Enables `Serialize`/`Deserialize` for all value types |
