# xsch

Schema-based validation for `xval` values. Define the shape and constraints of your data, then validate incoming values at runtime.

## Defining Schemas

Build schemas with a fluent API:

```rust
use xsch::*;

let name_schema = string().required().min(1).max(100);
let age_schema  = int().min(0).max(150);
let tags_schema = array().min(1).items(string());

let user_schema = object()
    .field("name", name_schema)
    .field("age", age_schema)
    .field("tags", tags_schema);
```

## Validating Values

```rust
use xsch::*;
use xval::ToValue;

let schema: Schema = string().required().min(3).into();

// passes
schema.validate(&"hello".to_value().into())?;

// fails — too short
schema.validate(&"hi".to_value().into()).unwrap_err();

// fails — null on a required field
schema.validate(&xval::valueof!(null).into()).unwrap_err();
```

## Nested Validation

Object and array schemas validate recursively:

```rust
use xsch::*;
use xval::ToValue;

let schema: Schema = object()
    .field("name", string().required())
    .field("scores", array().items(int()))
    .into();

let good = xval::valueof!({ "name": "alice", "scores": [90_i32, 85_i32] });
schema.validate(&good.into())?;

let bad = xval::valueof!({ "name": "alice", "extra": true });
schema.validate(&bad.into()).unwrap_err(); // unexpected field 'extra'
```

## Available Rules

| Rule | Applies to | Description |
|------|-----------|-------------|
| `required` | all | rejects null values |
| `equals` | all except array/object | must match a specific value |
| `options` | all except array/object | must be one of a set of values |
| `min` | string, array, number, int, float | minimum length or value |
| `max` | string, array, number, int, float | maximum length or value |
| `pattern` | string | regex match (requires `regex` feature) |
| `items` | array | schema applied to each array element |
| `fields` | object | named field schemas; rejects unexpected fields |

All schemas accept `null` by default. Add `required` to reject it.

## Schema Types

| Constructor | Matches |
|-------------|---------|
| `any()` | any value |
| `bool()` | booleans |
| `string()` | strings |
| `number()` | any number (int, uint, or float) |
| `int()` | signed integers |
| `float()` | floats |
| `array()` | arrays |
| `object()` | structs/objects |

## Derive Macro

Enable the `derive` feature to generate schemas from structs automatically:

```toml
[dependencies]
xsch = { version = "0.0.0", features = ["derive"] }
xval = { version = "0.0.0", features = ["derive"] }
```

```rust
use xval::derive::Value;
use xsch::derive::Validate;

#[derive(Clone, Default, Value, Validate)]
struct CreateUser {
    #[schema(required, min = 1)]
    name: String,

    #[schema(min = 0, max = 150)]
    age: i32,

    #[schema(options = [3, 4, 5])]
    level: i32,

    tags: Vec<String>,

    nickname: Option<String>,
}
```

Then validate instances directly:

```rust
use xsch::Validate;

let user = CreateUser {
    name: "alice".into(),
    age: 30,
    level: 4,
    tags: vec![],
    nickname: None,
};

let value = user.validate()?; // returns the validated xval::Value
```

### Supported Attributes

| Attribute | Description |
|-----------|-------------|
| `required` | field cannot be null |
| `min = N` | minimum value or length |
| `max = N` | maximum value or length |
| `equals = V` | must equal this value |
| `options = [A, B, C]` | must be one of these values |
| `pattern = "regex"` | string must match (requires `regex` feature) |

### Type Mapping

Rust types map to schema types automatically:

| Rust Type | Schema |
|-----------|--------|
| `String` | `string()` |
| `bool` | `bool()` |
| `i8`..`i128` | `int()` |
| `u8`..`u128` | `number()` |
| `f32`, `f64` | `float()` |
| `Vec<T>` | `array()` with item schema from `T` |
| `Option<T>` | schema from `T`, not required |

## Validation Errors

Failed validation returns a `ValidError` with the rule name, the path to the invalid value, and a descriptive message. Nested schemas produce a tree:

```json
{
  "name": "unknown",
  "path": "",
  "message": null,
  "errors": [
    { "name": "required", "path": "name", "message": "required", "errors": [] },
    { "name": "min", "path": "tags", "message": "length must be at least 1", "errors": [] }
  ]
}
```

## Serde

With the `serde` feature, schemas serialize as JSON/YAML with a `type` discriminator:

```json
{
  "type": "object",
  "fields": {
    "name": { "type": "string", "required": true, "min": 1 },
    "age": { "type": "int", "min": 0, "max": 150 }
  }
}
```

## Features

| Feature | Description |
|---------|-------------|
| `derive` | `#[derive(Validate)]` for struct validation via `xsch-derive` |
| `serde` | `Serialize`/`Deserialize` for schemas and errors |
| `regex` | enables the `pattern` rule on string schemas |
