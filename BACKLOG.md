# Backlog

## `xsch::Context` — add `From` impls for primitives

[libs/xsch/src/context.rs](libs/xsch/src/context.rs)

```rust
// before (~50+ call sites)
schema.validate(&true.to_value().into())
schema.validate(&42i32.to_value().into())
schema.validate(&"hello".to_value().into())

// after
schema.validate(&true.into())
schema.validate(&42i32.into())
schema.validate(&"hello".into())
```

Add `From<bool>`, `From<i8..i128>`, `From<u8..u128>`, `From<f32>`, `From<f64>`, `From<&str>`, `From<String>` for `Context` via macro, going through `Value::from()`. Keep existing `From<xval::Value> for Context`. The `Validator` trait signature stays unchanged to avoid churn in the internal rule/RuleSet call chain.
