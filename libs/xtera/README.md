# xtera

Template engine for Rust with interpolation, control flow, pipes, functions, and template composition. Templates can be parsed at runtime or compile time.

## Quick Start

```rust
use xtera::{Scope, Template};

let mut scope = Scope::new();
scope.set_var("name", "world");

let tpl = Template::parse("Hello {{ name }}!").unwrap();
assert_eq!(tpl.render(&scope).unwrap(), "Hello world!");
```

## Template Syntax

### Interpolation

Expressions inside `{{ }}` are evaluated and rendered:

```
Hello {{ name }}!
The answer is {{ x * 2 + 1 }}.
{{ name | upper }}
{{ val | slice:0:5 }}
{{ name | trim | upper }}
```

### Expressions

```
{{ x + y }}              arithmetic: + - * / %
{{ a == b }}             comparison: == != < <= > >=
{{ cond && other }}      logical:    && ||
{{ !flag }}              unary:      ! -
{{ user.name }}          member access
{{ items[0] }}           index access
{{ len(items) }}         function call
{{ [1, 2, 3] }}          array literal
{{ { name: "alice" } }}  object literal
```

### Conditionals

```
@if (show) {
    visible
} @else if (alt) {
    alternative
} @else {
    hidden
}
```

### Loops

```
@for (item of items; track item) {
    [{{ item }}]
}
```

### Pattern Matching

```
@match (color) {
    "red" => { R },
    "blue" => { B },
    _ => { ? }
}
```

### Template Inclusion

```
@include('header')
```

Renders a named template registered in the scope.

## Scope

The scope holds everything a template can reference — variables, pipes, functions, and other templates:

```rust
use xtera::{Scope, Template};

let mut scope = Scope::new();

scope.set_var("name", "alice");
scope.set_var("items", xval::valueof!([1_i64, 2_i64, 3_i64]));

scope.set_template("header", Template::parse("<h1>{{ title }}</h1>").unwrap());
let html = scope.render("header").unwrap();
```

## Custom Pipes and Functions

Pipes transform a value, functions produce one:

```rust
use xtera::{Pipe, Func, ast};

struct UpperPipe;
impl Pipe for UpperPipe {
    fn invoke(&self, val: &xval::Value, _args: &[xval::Value]) -> ast::Result<xval::Value> {
        Ok(xval::valueof!((val.as_string().as_str().to_uppercase())))
    }
}

struct LenFunc;
impl Func for LenFunc {
    fn invoke(&self, args: &[xval::Value]) -> ast::Result<xval::Value> {
        Ok(xval::valueof!((args[0].as_array().len() as i64)))
    }
}

scope.set_pipe("upper", UpperPipe);
scope.set_func("len", LenFunc);
```

Then use them in templates:

```
{{ name | upper }}
{{ len(items) }} items
```

## Compile-Time Templates

Enable the `derive` feature for the `render!` macro. Template syntax errors become compile errors:

```toml
[dependencies]
xtera = { version = "0.0.0", features = ["derive"] }
```

```rust
use xtera::{render, Scope};

let tpl = render! {
    @for (n of items; track n) {
        @if (n % 2 == 0) { "even" } @else { "odd" }
    }
};

let mut scope = Scope::new();
scope.set_var("items", xval::valueof!([1_i64, 2_i64, 3_i64]));
assert_eq!(tpl.render(&scope).unwrap(), "oddevenodd");
```

## Error Handling

Evaluation errors include byte-offset spans for reporting:

- `UndefinedVariable` — variable not in scope
- `UndefinedPipe` / `UndefinedTemplate` — pipe or template not registered
- `TypeError` — type mismatch (e.g. iterating a number)
- `DivisionByZero` — divide or modulo by zero
- `IndexOutOfBounds` — array index past end
- `NotIterable` / `NotCallable` — wrong type for operation

## Features

| Feature | Description |
|---------|-------------|
| `derive` | `render!` compile-time template macro via `xtera-derive` |
| `serde` | `Serialize`/`Deserialize` for `xval` value types |
