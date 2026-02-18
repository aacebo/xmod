# XSch

Schema validation for `xval` types. Provides a schema definition language that mirrors the `xval::Value` type hierarchy, enabling runtime type checking and input validation.

## Schema Types

| Schema | Matches |
|--------|---------|
| `any` | any `xval::Value` |
| `bool` | `Value::Bool` |
| `string` | `Value::String` |
| `number` | any `Value::Number` (int or float) |
| `int` | signed integers (`xval::Int`) |
| `float` | floating point (`xval::Float`) |
| `array` | `Value::Array` with typed items |
| `object` | `Value::Object` with typed fields |

All schemas accept `null` by default. Use the `required` rule to reject null values.

## Validation Rules

| Rule | Applies to | Description |
|------|-----------|-------------|
| `required` | all | rejects null values |
| `equals` | all except array/object | must match a specific value |
| `options` | all except array/object | must be one of a set of values |
| `min` | string, array, number, int, float | minimum length or value |
| `max` | string, array, number, int, float | maximum length or value |
| `items` | array | schema for each array element |
| `fields` | object | named field schemas; rejects unexpected fields |

## Rust API

Schemas use a fluent builder pattern:

```rust
use xsch::*;

string().required().min(3).max(100)
int().min(1).max(10)
float().options(&[1.0.into(), 2.5.into()])
array().min(1).items(string().into())
object()
    .field("name", string().required().into())
    .field("age", int().into())
    .field("tags", array().items(string().into()).into())
```

Validate values via the `Validate` trait:

```rust
use xval::AsValue;

let schema: Schema = string().required().min(3).into();
schema.validate(&"hello".as_value().into())?;
```

## Serde

With the `serde` feature enabled, schemas serialize using a `type` tag:

```json
{"type": "string", "required": true, "min": 3, "max": 100}
```

```json
{
  "type": "object",
  "required": true,
  "fields": {
    "name": {"type": "string", "required": true},
    "age": {"type": "int"}
  }
}
```

## YAML Examples

Schema definition for a workflow input:

```yaml
type: object
fields:
  name:
    type: string
    required: true
  age:
    type: int
  tags:
    type: array
    items:
      type: string
  address:
    type: object
    fields:
      street:
        type: string
      city:
        type: string
```

