# Backlog

## Simplify `.into()` / `.to_value()` verbosity

Following the pattern already applied to `ObjectSchema::field()` and `ArraySchema::items()` (accepting `T: ToSchema` instead of `Schema`), these additional call sites can be simplified.

### `AnySchema::equals` — accept `T: ToValue`

[libs/xsch/src/any.rs:16](libs/xsch/src/any.rs#L16)

```rust
// before
any().equals("hello".to_value())
any().equals(42i32.to_value())

// after
any().equals("hello")
any().equals(42i32)
```

Change signature from `fn equals(mut self, value: xval::Value)` to `fn equals<T: xval::ToValue>(mut self, value: T)`.

### `AnySchema::options` — accept `&[&dyn ToValue]`

[libs/xsch/src/any.rs:21](libs/xsch/src/any.rs#L21)

```rust
// before
any().options(&[1i32.to_value(), "test".to_value(), true.to_value()])

// after
any().options(&[&1i32, &"test", &true])
```

Change signature from `fn options(mut self, options: &[xval::Value])` to `fn options(mut self, options: &[&dyn xval::ToValue])`. Keeps mixed-type support via trait objects.

### `Context` — add `From` impls for primitives

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

### `Fields::field` — accept `T: ToSchema`

[libs/xsch/src/rule/fields.rs:31](libs/xsch/src/rule/fields.rs#L31)

```rust
// before
pub fn field(mut self, name: &str, schema: Schema) -> Self

// after
pub fn field<T: ToSchema>(mut self, name: &str, value: T) -> Self
```

Internal-only impact (called from `ObjectSchema::field`), keeps consistency with the public API.

### `StructExt` extension trait for `Struct::field`

[libs/xval/src/object/structs.rs](libs/xval/src/object/structs.rs)

```rust
// before (~15+ call sites)
s.field("name".into()).unwrap()
s.field("age".into()).unwrap()

// after
s.get("name").unwrap()
s.get("age").unwrap()
```

`Struct` is used as `dyn Struct` (via `Arc<dyn Struct>`), so the trait method can't be generic. Add an extension trait with a concrete `&str` parameter to stay object-safe:

```rust
pub trait StructExt: Struct {
    fn get(&self, key: &str) -> Option<&dyn ToValue> {
        self.field(key.into())
    }
}

impl<T: Struct + ?Sized> StructExt for T {}
```

Re-export from [libs/xval/src/lib.rs](libs/xval/src/lib.rs). Call sites in [structs.rs tests](libs/xval/src/object/structs.rs), [object mod.rs serde tests](libs/xval/src/object/mod.rs), and [xval-derive tests](libs/xval-derive/tests/).

### `xflux::Context::new` and `Context::set` — accept `impl ToValue`

[libs/xflux/src/context.rs:14](libs/xflux/src/context.rs#L14), [libs/xflux/src/context.rs:52](libs/xflux/src/context.rs#L52)

```rust
// before
Context::new(xval::valueof!({ "user": "alice" }))
ctx.set("token", xval::valueof!("abc123"))

// after
Context::new(map)
ctx.set("token", "abc123")
```

Change `fn new(input: Value)` to `fn new(input: impl xval::ToValue)` and `fn set(key, value: Value)` to `fn set(key, value: impl xval::ToValue)`. `Event::new` already accepts `impl ToValue` — these two methods are inconsistent with it.

### `xtera::Scope::set_var` — accept `impl ToValue`

[libs/xtera/src/scope.rs:43](libs/xtera/src/scope.rs#L43)

```rust
// before
scope.set_var("x", xval::valueof!(42_i64))
scope.set_var("name", xval::valueof!("world"))

// after
scope.set_var("x", 42_i64)
scope.set_var("name", "world")
```

Change `fn set_var(name, value: xval::Value)` to `fn set_var(name, value: impl xval::ToValue)`. ~8 test call sites in [scope.rs](libs/xtera/src/scope.rs).

### `xpath::Path` methods — accept `impl Into<Ident>`

[libs/xpath/src/path.rs:41](libs/xpath/src/path.rs#L41), [libs/xpath/src/path.rs:50](libs/xpath/src/path.rs#L50), [libs/xpath/src/path.rs:56](libs/xpath/src/path.rs#L56)

```rust
// before
path.push(Ident::key("name"))
path.child(Ident::key("name"))
path.child(i.into())

// after
path.push("name")
path.child("name")
path.child(i)
```

Change `push(ident: Ident)`, `child(ident: Ident)`, and `peer(ident: Ident)` to accept `impl Into<Ident>`. `From<&str>`, `From<String>`, and `From<usize>` already exist for `Ident`.
