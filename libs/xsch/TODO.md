# xsch: Joi-like Schema Validation for xval::Value

## Context

The `xsch` crate currently has a `Validate` trait, `ValidError`, and empty `BoolSchema`/`BoolSchemaBuilder` skeletons. The goal is to build it into a full Joi-style schema builder/validation library that validates `xval::Value` types using a fluent builder API.

## Design

### API Style

Fluent builder pattern with top-level free functions as entry points:

```rust
// Scalar
xsch::string().min(3).max(30).pattern("[a-z]+").build()
xsch::number().min(0).max(100).build()
xsch::uint().min(0).max(255).build()
xsch::bool().equals(true).build()

// Composite
xsch::array().items(xsch::string().build()).min(1).max(10).build()
xsch::object()
    .field("name", xsch::string().min(1).build())
    .field("age", xsch::uint().build())
    .required(vec!["name".into(), "age".into()])
    .build()
xsch::tuple()
    .item(xsch::string().build())
    .item(xsch::number().build())
    .build()

// Meta
xsch::optional(xsch::string().build()).build()
xsch::one_of()
    .try_schema(xsch::string().build())
    .try_schema(xsch::number().build())
    .build()

// Builders also implement Validate directly (no .build() needed)
xsch::string().min(3).validate(xval::Value::from_str("hi"))

// Default values (injected during validation when field is missing/null)
xsch::object()
    .field("port", xsch::uint().default(xval::Value::from_u16(8080)).build())
    .build()
```

### Schema Enum

A unified `Schema` enum wraps all concrete types, implements `Validate`, and supports serde (`#[serde(tag = "type")]`):

```rust
pub enum Schema {
    Null(NullSchema),
    Bool(BoolSchema),
    String(StringSchema),
    Number(NumberSchema),
    Int(IntSchema),
    UInt(UIntSchema),
    Float(FloatSchema),
    Array(ArraySchema),
    Object(ObjectSchema),
    Tuple(TupleSchema),
    Optional(OptionalSchema),
    OneOf(OneOfSchema),
}
```

### Validate Trait

Keep owned value signature to support coercion and default value injection:

```rust
pub trait Validate {
    fn validate(&self, input: xval::Value) -> Result<xval::Value, error::ValidError>;
}
```

This allows schemas to transform values (e.g. inject defaults for missing fields, coerce types) while validating.

## File Structure

```
libs/xsch/src/
  lib.rs                  -- Validate trait, free functions, re-exports
  error/
    mod.rs                -- re-exports
    valid.rs              -- ValidError (enhanced with constructors + Display/Error)
  schema/
    mod.rs                -- Schema enum, Validate dispatch
    null.rs               -- NullSchema
    bool.rs               -- BoolSchema + BoolSchemaBuilder
    string.rs             -- StringSchema + StringSchemaBuilder
    number.rs             -- NumberSchema + NumberSchemaBuilder
    int.rs                -- IntSchema + IntSchemaBuilder
    uint.rs               -- UIntSchema + UIntSchemaBuilder
    float.rs              -- FloatSchema + FloatSchemaBuilder
    array.rs              -- ArraySchema + ArraySchemaBuilder
    object.rs             -- ObjectSchema + ObjectSchemaBuilder
    tuple.rs              -- TupleSchema + TupleSchemaBuilder
    optional.rs           -- OptionalSchema + OptionalSchemaBuilder
    one_of.rs             -- OneOfSchema + OneOfSchemaBuilder
```

Delete existing `src/bool_schema.rs`.

## Schema Types and Constraints

| Type | Constraints |
|------|-------------|
| **NullSchema** | (none - just checks `is_null()`) |
| **BoolSchema** | `equals: Option<bool>`, `default: Option<Value>` |
| **StringSchema** | `min_len`, `max_len`, `pattern` (regex), `one_of` (allowlist), `default: Option<Value>` |
| **NumberSchema** | `min: Option<f64>`, `max: Option<f64>`, `one_of: Option<Vec<f64>>`, `default: Option<Value>` |
| **IntSchema** | `min: Option<i64>`, `max: Option<i64>`, `one_of: Option<Vec<i64>>`, `default: Option<Value>` |
| **UIntSchema** | `min: Option<u64>`, `max: Option<u64>`, `one_of: Option<Vec<u64>>`, `default: Option<Value>` |
| **FloatSchema** | `min: Option<f64>`, `max: Option<f64>`, `one_of: Option<Vec<f64>>`, `default: Option<Value>` |
| **ArraySchema** | `items: Option<Box<Schema>>`, `min_len`, `max_len`, `default: Option<Value>` |
| **ObjectSchema** | `fields: HashMap<String, Schema>`, `required: Option<Vec<String>>` |
| **TupleSchema** | `items: Vec<Schema>` (positional) |
| **OptionalSchema** | `schema: Box<Schema>` (allows null or inner match) |
| **OneOfSchema** | `schemas: Vec<Schema>` (tries each, first match wins) |

### Default Value Behavior

When a schema has a `default` set and the input is `Value::Null`, the default value is returned instead. The default is also validated against the schema's own constraints. For `ObjectSchema`, if a field is missing/null and its field schema has a default, the default is injected into the output.

## Error Handling

Enhance `ValidError` with:
- `new(rule, path, message)` constructor
- `with_errors(rule, path, message, children)` constructor
- `with_path_prefix(prefix)` for nested path building
- `Display` and `std::error::Error` impls

Path building is bottom-up: leaf validators set `path: ""`, parent validators prepend field names/indices via `with_path_prefix`. Result: paths like `"address.street"`, `"tags[2]"`, `"items[0].name"`.

## Dependencies

Add to `Cargo.toml`:
- `regex` (optional, behind `regex` feature flag) for `StringSchema::pattern`
- The `pattern()` builder method is only available when `regex` feature is enabled

## Implementation Order

### Phase 1: Foundation
1. Enhance `ValidError` with constructors, `with_path_prefix`, Display/Error impls
2. Create `schema/mod.rs` with `Schema` enum (start with Null + Bool)
3. Implement `NullSchema`, `BoolSchema` + builder
4. Update `lib.rs`: remove old `bool_schema` module, add `schema` module + free functions
5. Tests for Null and Bool

### Phase 2: Scalar Types
6. `StringSchema` + builder (min, max, pattern, one_of)
7. `NumberSchema` + builder
8. `IntSchema`, `UIntSchema`, `FloatSchema` + builders
9. Tests for all scalar schemas

### Phase 3: Composite Types
10. `ArraySchema` + builder (items, min, max)
11. `ObjectSchema` + builder (fields, required)
12. `TupleSchema` + builder (positional items)
13. Tests for nested validation and error paths

### Phase 4: Meta Types
14. `OptionalSchema` + builder
15. `OneOfSchema` + builder
16. Tests

### Phase 5: Polish
17. Serde derives on all schema types
18. Top-level free functions finalized
19. Integration tests

## Verification

1. `cargo build -p xsch` compiles clean
2. `cargo test -p xsch` passes all unit tests
3. `cargo build -p xsch --features serde` compiles with serde
4. Integration test: build a nested struct schema with arrays and optional fields, validate both passing and failing values, verify error paths are correct
