# xtera

A template engine for Rust with runtime and compile-time parsing. Supports interpolation, control flow directives, pipes, functions, and template composition.

## Quick Start

```rust
use xtera::{Scope, Template};

let mut scope = Scope::new();
scope.set_var("name", xval::valueof!("world"));

let tpl = Template::parse("Hello {{ name }}!").unwrap();
assert_eq!(tpl.render(&scope).unwrap(), "Hello world!");
```

## Template Syntax

### Text and Interpolation

Plain text passes through unchanged. Expressions inside `{{ }}` are evaluated and rendered:

```
Hello {{ name }}!
The answer is {{ x * 2 + 1 }}.
```

### Expressions

```
{{ x + y }}              Arithmetic: + - * / %
{{ a == b }}             Comparison: == != < <= > >=
{{ cond && other }}      Logical: && || (short-circuit)
{{ !flag }}              Unary: ! -
{{ user.name }}          Member access
{{ items[0] }}           Index access
{{ len(items) }}         Function calls
{{ name | upper }}       Pipes
{{ val | slice:0:5 }}    Pipes with arguments
{{ [1, 2, 3] }}          Array literals
{{ { name: "alice" } }}  Object literals
```

Pipes can be chained: `{{ name | trim | upper }}`.

### Control Flow

**Conditionals:**

```
@if (show) {
    visible
} @else if (alt) {
    alternative
} @else {
    hidden
}
```

**Loops:**

```
@for (item of items; track item) {
    [{{ item }}]
}
```

**Pattern matching:**

```
@match (color) {
    "red" => { R },
    "blue" => { B },
    _ => { ? }
}
```

**Template inclusion:**

```
@include('header')
```

Renders a named template registered in the scope.

## Scope API

`Scope` holds variables, pipes, functions, and templates:

```rust
let mut scope = Scope::new();

// Variables (any xval::Value)
scope.set_var("name", xval::valueof!("alice"));
scope.set_var("items", xval::valueof!([1_i64, 2_i64, 3_i64]));

// Pipes (implement the Pipe trait)
scope.set_pipe("upper", UpperPipe);

// Functions (implement the Func trait)
scope.set_func("len", LenFunc);

// Named templates
scope.set_template("header", Template::parse("<h1>{{ title }}</h1>").unwrap());

// Render a named template
let html = scope.render("header").unwrap();
```

### Custom Pipes and Functions

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
```

## Compile-Time Templates

Enable the `derive` feature for the `render!` macro, which parses templates at compile time:

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

The `render!` macro produces a `Template` with the same behavior as `Template::parse`, but template syntax errors are caught at compile time.

## Error Handling

All evaluation errors include byte-offset spans for error reporting:

- `UndefinedVariable` - variable not in scope
- `UndefinedPipe` / `UndefinedTemplate` - pipe or template not registered
- `TypeError` - type mismatch (e.g. iterating a number)
- `DivisionByZero` - divide or modulo by zero
- `IndexOutOfBounds` - array index out of range
- `NotIterable` / `NotCallable` - wrong type for operation

## Features

| Feature | Description |
|---------|-------------|
| `derive` | Enables the `render!` compile-time template macro via `xtera-derive` |
