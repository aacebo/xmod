# XSch

Schema validation for `xval` types. Provides a schema definition language that mirrors the `xval::Value` type hierarchy, enabling runtime type checking and input validation for `xflux` workflows.

## Schema Types

| Schema | Matches |
|--------|---------|
| `null` | `Value::Null` |
| `bool` | `Value::Bool` |
| `string` | `Value::String` |
| `number` | any `Value::Number` |
| `int` | signed integers (`i8`, `i16`, `i32`, `i64`) |
| `uint` | unsigned integers (`u8`, `u16`, `u32`, `u64`) |
| `float` | floating point (`f32`, `f64`) |
| `array` | `Value::Object(Array)` with typed items |
| `struct` | `Value::Object(Struct)` with typed fields |
| `tuple` | `Value::Object(Tuple)` with positional types |
| `optional` | value or `null` |
| `one_of` | union of multiple schemas |

## Examples

> schema definition for a workflow input

```yaml
type: struct
fields:
  name:
    type: string
  age:
    type: uint
  tags:
    type: array
    items:
      type: string
  address:
    type: optional
    schema:
      type: struct
      fields:
        street:
          type: string
        city:
          type: string
```

> usage in an xflux workflow

```yaml
version: "0.0.1"
name: greet-user
display_name: "Greet User"
input:
  type: struct
  fields:
    name:
      type: string
    greeting:
      type: optional
      schema:
        type: string
actions:
  - id: greet
    type: log::info
    input: "{{ $greeting ?? 'Hello' }}, {{ $name }}!"
```
