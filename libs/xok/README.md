# xok

Standardized error handling for the xmod workspace. Every crate in the workspace uses `xok` to define its errors, giving consumers a consistent way to inspect error name, origin module, severity, and classification.

## Defining Errors

Implement `XError` on your error type to plug into the shared error system:

```rust
use xok::{XError, Code, Severity};

struct MyError { msg: String }

impl std::fmt::Display for MyError { /* ... */ }
impl std::error::Error for MyError {}

impl XError for MyError {
    fn name(&self) -> &'static str { "MyError" }
    fn module(&self) -> &'static str { module_path!() }
    fn code(&self) -> Code { Code::BadArgument }
    fn severity(&self) -> Severity { Severity::High }
}
```

## Using Results

Functions that can fail return `xok::Result<T>`, which boxes any `XError` implementor:

```rust
fn do_work() -> xok::Result<String> {
    Err(MyError { msg: "something broke".into() }.boxed())
}
```

## Inspecting Errors

```rust
match do_work() {
    Err(e) => {
        println!("{} in {}: {}", e.name(), e.module(), e.message());
        println!("code={:?} severity={}", e.code(), e.severity());
    }
    Ok(v) => println!("{v}"),
}
```

## Error Codes

`Code` classifies the failure: `Internal`, `NotFound`, `BadArgument`, `UnAuthorized`, `Timeout`, `Conflict`, `Duplicate`.

## Severity

`Severity` indicates impact: `Low`, `Medium`, `High`.

## Features

| Feature | Description |
|---------|-------------|
| `serde` | Enables `Serialize`/`Deserialize` for `Code` and `Severity` |
