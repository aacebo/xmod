# xwire

Compile-time service wiring for Rust. A zero-cost alternative to C#'s `IServiceProvider` that uses the type system to resolve dependencies at compile time instead of runtime dictionary lookups.

## Motivation

C#'s dependency injection relies on `IServiceProvider`, which resolves services by type at runtime:

```csharp
var logger = provider.GetService<ILogger>();
var db = provider.GetService<IDbConnection>();
```

This involves a `Dictionary<Type, object>` under the hood — allocation, type erasure, possible null returns, and no compile-time guarantees that a service was registered.

Rust can encode the entire service set in the type system. Resolution becomes a compile-time check, and the generated code compiles down to direct field access with zero overhead.

## `Accessor<T>`

The core trait. A type implementing `Accessor<T>` declares that it can provide access to a value of type `T`.

```rust
pub trait Accessor<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
    fn set(&mut self, value: T);
}
```

A single container can implement `Accessor` for many types simultaneously. Functions express their dependencies as trait bounds:

```rust
fn handle_request(ctx: &mut (impl Accessor<Logger> + Accessor<DbConn>)) {
    let logger = ctx.get();       // inferred as &Logger
    let db: &DbConn = ctx.get();  // or explicit
    // ...
}
```

The compiler verifies that every required service exists. If a bound isn't satisfied, you get a compile error — not a runtime null or exception.

## `#[derive(Wire)]`

The proc macro generates `Accessor<T>` impls for each field of a struct.

```rust
#[derive(Wire)]
struct AppCtx {
    logger: Logger,
    db: DbConn,
    cache: CacheClient,
}
```

This generates:

```rust
impl ::xwire::Accessor<Logger> for AppCtx {
    fn get(&self) -> &Logger { &self.logger }
    fn get_mut(&mut self) -> &mut Logger { &mut self.logger }
    fn set(&mut self, value: Logger) { self.logger = value; }
}

impl ::xwire::Accessor<DbConn> for AppCtx {
    fn get(&self) -> &DbConn { &self.db }
    fn get_mut(&mut self) -> &mut DbConn { &mut self.db }
    fn set(&mut self, value: DbConn) { self.db = value; }
}

impl ::xwire::Accessor<CacheClient> for AppCtx {
    fn get(&self) -> &CacheClient { &self.cache }
    fn get_mut(&mut self) -> &mut CacheClient { &mut self.cache }
    fn set(&mut self, value: CacheClient) { self.cache = value; }
}
```

The macro emits a compile error if two fields share the same type, since `Accessor<T>` can only be implemented once per `T`.

Only structs with named fields are supported.

## Unique types and newtypes

Since `Accessor<T>` is parameterized by type, each concrete type can appear at most once in a wired struct. This is a feature — it forces meaningful, distinct types and prevents accidentally grabbing the wrong `u32`.

Use `#[repr(transparent)]` newtypes:

```rust
#[repr(transparent)]
struct Port(u16);

#[repr(transparent)]
struct MaxRetries(u32);

#[derive(Wire)]
struct Config {
    port: Port,
    max_retries: MaxRetries,
}
```

The newtypes compile away entirely. The generated `Accessor` impls are direct field access with no indirection.

## `#[wire(flatten)]`

For hierarchical composition — like C#'s scoped service containers — a field can be flattened so that the outer struct inherits the inner struct's `Accessor` impls.

```rust
#[derive(Wire)]
struct AppCtx {
    logger: Logger,
    db: DbConn,
}

#[derive(Wire)]
struct RequestCtx {
    #[wire(flatten)]
    app: AppCtx,
    user: CurrentUser,
}
```

`RequestCtx` now implements:
- `Accessor<Logger>` — delegated through `self.app`
- `Accessor<DbConn>` — delegated through `self.app`
- `Accessor<CurrentUser>` — from its own field

The generated forwarding code:

```rust
impl ::xwire::Accessor<Logger> for RequestCtx {
    fn get(&self) -> &Logger { self.app.get() }
    fn get_mut(&mut self) -> &mut Logger { self.app.get_mut() }
    fn set(&mut self, value: Logger) { self.app.set(value); }
}
```

A compile error is emitted if a flattened type and a direct field would produce overlapping `Accessor<T>` impls for the same `T`.

The flatten field itself does **not** get an `Accessor` impl — it's structurally transparent.

## Usage patterns

### Generic functions with where clauses

```rust
fn process<C>(ctx: &C)
where
    C: Accessor<Logger> + Accessor<DbConn>,
{
    let logger: &Logger = ctx.get();
    let db: &DbConn = ctx.get();
}
```

### Optional services

Wrap the field type in `Option<T>`. The generated impl is `Accessor<Option<T>>`:

```rust
#[derive(Wire)]
struct Ctx {
    cache: Option<CacheClient>,
}

fn maybe_cache(ctx: &impl Accessor<Option<CacheClient>>) {
    if let Some(cache) = ctx.get() {
        // ...
    }
}
```

### Trait object services

Use `Box<dyn Trait>` fields for runtime polymorphism. The container is still zero-cost — the accessor is direct field access, even though the service itself uses dynamic dispatch:

```rust
#[derive(Wire)]
struct Ctx {
    logger: Box<dyn Log>,
    db: Box<dyn Database>,
}

fn handle(ctx: &impl Accessor<Box<dyn Log>>) {
    ctx.get().log("request received");
}
```

## Zero-cost guarantees

- No `HashMap`, no `TypeId`, no `Box<dyn Any>`
- All resolution is compile-time trait resolution via monomorphization
- Generated code is direct field access — identical to hand-written `self.field`
- `#[repr(transparent)]` newtypes have no runtime representation
- Missing services are compile errors, not runtime failures

## Comparison with C# `IServiceProvider`

| Aspect | C# IServiceProvider | xwire |
|---|---|---|
| Resolution | Runtime dictionary lookup | Compile-time trait resolution |
| Missing service | Runtime null / exception | Compile error |
| Cost | Allocation + type erasure | Zero — direct field access |
| Service uniqueness | By type + lifetime scope | By type — enforced at compile time |
| Composition | Scoped containers (runtime) | `#[wire(flatten)]` (compile-time) |
| Polymorphism | Interfaces | Trait objects (`Box<dyn T>`) |

## Crate structure

### xwire

- `Accessor<T>` trait definition
- Feature-gated re-export of `xwire-derive` under `derive` feature

### xwire-derive

- `#[derive(Wire)]` proc macro
- Parses `#[wire(flatten)]` attributes
- Emits `impl ::xwire::Accessor<T>` blocks with fully qualified paths
- Compile errors via `syn::Error::new_spanned` for: unions, enums, tuple structs, duplicate field types, overlapping flatten types
