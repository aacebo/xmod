# Code Review: xmod Workspace

## Architecture Overview

The workspace forms a layered ecosystem:

```
xpath (path navigation) ─┐
xok  (error traits)  ────┤
                          ├── xval (dynamic values) ──── xval-derive
                          ├── xsch (schema validation) ── xsch-derive
                          ├── xtera (templates) ──────── xtera-derive
                          └── xpipe (lazy pipelines)
```

Overall the codebase is clean, well-organized, and shows strong Rust fundamentals. Below are the findings per crate, followed by cross-cutting themes.

---

## 2. xval-derive — Value Derive Macros

**Strengths:** Clean array-based struct iteration pattern, correct `::xval::` path hygiene.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 2.1 | ✅ | **Bug** | Generics completely ignored — `Wrapper<T>` won't compile | [lib.rs:16](libs/xval-derive/src/lib.rs#L16) |
| 2.2 | ⬜ | **Bug** | `len` counts all fields but iteration skips unnamed (tuple structs break) | [lib.rs:17-18](libs/xval-derive/src/lib.rs#L17) |
| 2.3 | ⬜ | **Design** | Hidden `Clone` requirement baked into struct derive (`self.clone()`) | [lib.rs:23](libs/xval-derive/src/lib.rs#L23) |
| 2.4 | ⬜ | **UX** | Unions silently produce empty output instead of `compile_error!` | [lib.rs:11](libs/xval-derive/src/lib.rs#L11) |
| 2.5 | ⬜ | **Bug** | Enum named-variant uses `HashMap` — loses field declaration order | [lib.rs:79](libs/xval-derive/src/lib.rs#L79) |
| 2.6 | ⬜ | **Hygiene** | `std::any::TypeId` not fully qualified (should be `::std::`) | [lib.rs:33](libs/xval-derive/src/lib.rs#L33) |

### Details

#### 2.1 — Generics not handled

The generated `impl` block uses only `#ident` without incorporating `input.generics`. Any generic struct will fail to compile.

**Fix:** Extract generics with `input.generics.split_for_impl()` and thread `(impl_generics, ty_generics, where_clause)` into generated code.

#### 2.2 — Tuple struct len/fields mismatch

```rust
let len = data.fields.len();        // counts ALL fields
let fields: Vec<syn::Ident> = data.fields.iter().filter_map(|f| f.ident.clone()).collect();
```

For a tuple struct with 2 elements, `len()` returns 2 but `items()` yields 0 entries.

**Fix:** Either handle tuple structs properly (access via `self.0`, `self.1`) or emit `compile_error!`.

#### 2.3 — Hidden `Clone` requirement

```rust
fn to_value(&self) -> ::xval::Value {
    ::xval::Value::from_struct(self.clone())
}
```

Users must also `#[derive(Clone)]` but get no clear error message if they don't.

**Fix:** Build the representation by iterating fields (like the enum path does) rather than cloning `self`.

#### 2.4 — No `syn::Error` diagnostics

The macro never uses `syn::Error::new_spanned(...)`. All failure modes produce silent empty output or rely on downstream type errors. Unsupported cases (unions, tuple structs) should emit clear `compile_error!` messages.

---

## 3. xtera — Template Engine

**Strengths:** Clean lex-parse-eval architecture, excellent `Span` design with `Arc<str>`, comprehensive test suite, Pratt precedence parser, Serde round-trip support.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 3.1 | ⬜ | **Bug** | `Scope::render` panics on missing template instead of returning `Err` | [scope.rs:64](libs/xtera/src/scope.rs#L64) |
| 3.2 | ⬜ | **Bug** | `@include` has no recursion depth limit — stack overflow possible | [include.rs:31](libs/xtera/src/ast/node/include.rs#L31) |
| 3.3 | ⬜ | **Bug** | `parse_block_body` doesn't detect unclosed `}` at EOF | parser.rs |
| 3.4 | ⬜ | **Perf** | Per-token logos lexer instantiation — O(n^2) for expressions | lexer.rs |
| 3.5 | ⬜ | **Perf** | `ForNode::render` clones entire `Scope` per iteration | [for_node.rs:29](libs/xtera/src/ast/node/for_node.rs#L29) |
| 3.6 | ⬜ | **Design** | `Node::span()`/`Expr::span()` return owned `Span` (should be `&Span`) | [node/mod.rs](libs/xtera/src/ast/node/mod.rs), [expr/mod.rs](libs/xtera/src/ast/expr/mod.rs) |
| 3.7 | ⬜ | **Security** | No HTML auto-escaping — XSS risk in HTML contexts | [interp.rs:12](libs/xtera/src/ast/node/interp.rs#L12) |
| 3.8 | ⬜ | **Dead code** | `ForNode.track` field is parsed but never used during rendering | [for_node.rs:10](libs/xtera/src/ast/node/for_node.rs#L10) |

### Details

#### 3.1 — `Scope::render` panics

```rust
pub fn render(&self, name: &str) -> ast::Result<String> {
    self.template(name)
        .expect("template not found")  // PANICS
        .render(self)
}
```

**Fix:** Return `Err(EvalError::UndefinedTemplate(...))`.

#### 3.2 — Unbounded `@include` recursion

`IncludeNode::render` calls `tpl.render(scope)` which can include another template, which can include the first, leading to stack overflow.

**Fix:** Add a recursion depth counter (e.g., a `Cell<usize>` in `Scope`). Error at depth ~64.

#### 3.4 — O(n^2) expression parsing

Every `scan_expr` call creates a fresh `Token::lexer()` over the remaining input, reads one token, then discards the lexer.

**Fix:** Maintain a persistent logos lexer for the expression context, or buffer all expression tokens in one pass.

#### 3.5 — Scope cloning in `@for`

```rust
let mut inner = scope.clone();
```

`Scope` contains four `BTreeMap`s. For n iterations over a scope with m variables, this is O(n*m) allocation.

**Fix:** Use a child-scope / scope-stack pattern. O(1) per iteration for the new binding.

### Missing Features

- No whitespace control (`{{- expr -}}`)
- No `@let` / variable assignment
- No `@empty` for `@for` loops
- No template inheritance / blocks (`@extends` / `@block`)
- `Pipe` and `Func` traits are not `Send + Sync` (limits async usage)

---

## 4. xtera-derive — Compile-time Template Macro

**Strengths:** Correct Pratt parsing, good error messages, `{{ }}` double-brace validation, thorough test suite (17 tests), fully qualified paths.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 4.1 | ⬜ | **UX** | All spans are `(0,0)` — runtime errors have no useful source location | [parse.rs](libs/xtera-derive/src/parse.rs) throughout |
| 4.2 | ⬜ | **Bug** | Missing trailing-token validation in parenthesized exprs and `@include` | [parse.rs:619](libs/xtera-derive/src/parse.rs#L619), [parse.rs:315](libs/xtera-derive/src/parse.rs#L315) |
| 4.3 | ⬜ | **Bug** | Duplicate `_` arms in `@match` silently overwritten | [parse.rs:283](libs/xtera-derive/src/parse.rs#L283) |
| 4.4 | ⬜ | **Design** | Duplicated precedence table (vs runtime parser) with no cross-validation test | [parse.rs:27-34](libs/xtera-derive/src/parse.rs#L27) |
| 4.5 | ⬜ | **Bug** | `@for` header does not validate that all tokens are consumed after `track` expr | [parse.rs:251](libs/xtera-derive/src/parse.rs#L251) |

### Details

#### 4.1 — All spans are `(0,0)`

Every AST node gets `Span::new(0, 0, src.clone())`. Runtime errors from macro-generated templates always point to position `0..0`, which is useless for debugging.

#### 4.2 — Missing trailing-token validation

After `parse_expr(&content)` in parenthesized expressions, `@include`, and `@for` track expressions, there is no check that all content was consumed. Extra tokens are silently ignored.

**Fix:** Add `if !content.is_empty() { return Err(content.error("unexpected tokens")); }` after each delimited parse.

#### 4.3 — No compile-fail tests

Only happy-path tests exist. Should add `trybuild` tests for invalid syntax to verify error messages.

---

## 5. xsch — Schema Validation

**Strengths:** Composable builder API (`string().required().min(3).max(10)`), error accumulation, path tracking, structured errors.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 5.1 | ⬜ | **Bug** | Type checking happens AFTER rules — rules can operate on wrong types (panics) | All typed schemas (e.g., [bool.rs](libs/xsch/src/bool.rs), [string.rs](libs/xsch/src/string.rs)) |
| 5.2 | ⬜ | **Bug** | `Equals` and `Options` fire on null, breaking null-by-default contract | [equals.rs:34](libs/xsch/src/rule/equals.rs#L34), [options.rs:40](libs/xsch/src/rule/options.rs#L40) |
| 5.3 | ⬜ | **Bug** | `Pattern` calls `.as_str()` on non-strings — will panic | [pattern.rs:39](libs/xsch/src/rule/pattern.rs#L39) |
| 5.4 | ⬜ | **Design** | `Items` short-circuits on first error but `Fields` accumulates — inconsistent | [items.rs:43](libs/xsch/src/rule/items.rs#L43) |
| 5.5 | ⬜ | **Dead code** | `Phase` enum and `Validate` trait are defined but never used | [phase.rs](libs/xsch/src/phase.rs), [lib.rs:60](libs/xsch/src/lib.rs#L60) |
| 5.6 | ⬜ | **Testing** | `debug_assert!` used in tests — stripped in release mode | [error.rs:100](libs/xsch/src/error.rs#L100) |
| 5.7 | ⬜ | **Design** | `Min`/`Max` error messages don't distinguish length vs value | [min.rs:37](libs/xsch/src/rule/min.rs#L37), [max.rs:37](libs/xsch/src/rule/max.rs#L37) |

### Details

#### 5.1 — Type checking after rules (critical)

In every typed schema, rules execute before the type check:

```rust
fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
    let value = self.0.validate(ctx)?;  // run all rules first
    if !value.is_null() && !value.is_bool() {  // then check type
        return Err(ctx.error("expected bool"));
    }
    Ok(value)
}
```

If you have `int().min(5)` and pass a string, `Min::validate` sees `is_number() == false` and `is_string() == true`, so it compares string length against 5 — completely wrong semantics. Worse, `Pattern` will call `.as_str()` on a non-string and panic.

**Fix:** Check type first:

```rust
fn validate(&self, ctx: &Context) -> Result<xval::Value, ValidError> {
    if ctx.value.is_null() {
        return self.0.validate(ctx); // only run presence rules
    }
    if !ctx.value.is_string() {
        return Err(ctx.error("expected string"));
    }
    self.0.validate(ctx)
}
```

#### 5.2 — `Equals` and `Options` fire on null

Rules like `Min`, `Max`, `Items`, `Fields`, and `Pattern` all guard with `if !ctx.value.is_null()`, but `Equals` and `Options` do not. This means `string().equals("hello")` rejects null even without `.required()`, breaking the null-by-default contract.

**Fix:** Add `if ctx.value.is_null() { return Ok(ctx.value.clone()); }` at the top of both validators.

#### 5.6 — `debug_assert!` in tests

```rust
debug_assert!(output.contains("Error[required]"));
```

These assertions are stripped in `cargo test --release`.

**Fix:** Replace with `assert!`.

### Missing Validation Rules

- `unique_items` for arrays
- `additional_fields` / `additional_properties` for objects (currently unexpected fields are always rejected)
- Conditional validation (`oneOf`/`anyOf`/`allOf`)
- `exclusive_min`/`exclusive_max`
- Custom validation functions / `Refine` phase

---

## 6. xsch-derive — Schema Derive Macros

**Strengths:** Good `syn::Error`-based diagnostics, clean builder-chain code gen, comprehensive rule parsing with clear error messages.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 6.1 | ⬜ | **High** | No `Option<T>` support — extremely common Rust pattern falls back to `any()` | [schema_type.rs:17](libs/xsch-derive/src/schema_type.rs#L17) |
| 6.2 | ⬜ | **High** | Generics not handled | [lib.rs:18](libs/xsch-derive/src/lib.rs#L18) |
| 6.3 | ⬜ | **Medium** | Enums/unions silently produce no output | [lib.rs:13](libs/xsch-derive/src/lib.rs#L13) |
| 6.4 | ⬜ | **Medium** | Unsigned integers map to `any()` — no type checking | [schema_type.rs:42](libs/xsch-derive/src/schema_type.rs#L42) |
| 6.5 | ⬜ | **Medium** | `options` not supported for `Bool` or `Any` despite runtime support | [lib.rs:125](libs/xsch-derive/src/lib.rs#L125) |
| 6.6 | ⬜ | **Low** | `required = false` is silently ignored — always emits `required()` | [lib.rs:94](libs/xsch-derive/src/lib.rs#L94) |
| 6.7 | ⬜ | **Low** | Only first `#[field]` attribute per field is processed | [lib.rs:28](libs/xsch-derive/src/lib.rs#L28) |
| 6.8 | ⬜ | **Low** | `pattern` emitted unconditionally but requires `xsch/regex` feature at runtime | [lib.rs:136](libs/xsch-derive/src/lib.rs#L136) |

### Details

#### 6.1 — No `Option<T>` / `Vec<T>` support

`SchemaType::from_type` only handles simple idents via `get_ident()`. Any path with segments (`Option<String>`, `Vec<i32>`, `std::string::String`) falls through to `Self::Any`.

**Fix (prioritized):**
1. Handle `Option<T>` by extracting inner type and generating a non-required schema
2. Handle `Vec<T>` by generating `::xsch::array()` with `items` schema
3. Handle `&str` as string
4. Handle qualified paths by checking last segment

#### 6.4 — Unsigned integers -> `any()`

```rust
"u8" | "u16" | "u32" | "u64" | "u128" | "usize" => Self::Any,
```

A `count: u32` field gets zero type checking at validation time.

**Fix:** Map to `::xsch::int()` or `::xsch::number()`.

---

## 7. xpath — Path Navigation

**Strengths:** Clean, focused design. Correct parsing. Good serde integration. Solid test coverage for parse/display.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 7.1 | ⬜ | **Design** | `From<&str>` panics on invalid input — violates infallible `From` contract | [path.rs:64](libs/xpath/src/path.rs#L64) |
| 7.2 | ⬜ | **Testing** | `push`, `pop`, `child`, `peer`, `iter` have zero test coverage | — |
| 7.3 | ⬜ | **Design** | No `FromStr`/`IntoIterator`/`TryFrom` standard trait impls | — |
| 7.4 | ⬜ | **Design** | Numeric keys impossible — `"42"` always becomes `Index(42)` | [ident.rs:14](libs/xpath/src/ident.rs#L14) |

### Details

#### 7.1 — Panicking `From<&str>`

```rust
impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self::parse(value).unwrap()
    }
}
```

`From` conversions are expected to be infallible in Rust. Passing `"/a"`, `"a//b"`, or `"a/"` will panic.

**Fix:** Replace with `TryFrom<&str>` / `FromStr`. Add `IntoIterator` for `&Path`.

#### 7.4 — Numeric key ambiguity

```rust
pub fn parse(src: &str) -> Self {
    if let Ok(index) = src.parse::<usize>() {
        return Self::Index(index);
    }
    Self::Key(src.into())
}
```

Any purely numeric segment becomes an `Index`. No way to represent a key that happens to be a number. Should be documented.

### Missing

- `first()` method (has `last()` but no `first()`)
- `get(index) -> Option<&Ident>` for safe indexed access
- `parent()` method (inverse of `child`)
- Clippy nit: `split("/")` should be `split('/')`

---

## 8. xpipe — Lazy Pipeline Library

**Strengths:** Clean extension-trait pattern (`.map()`, `.filter()`, etc.), good operator set, well-designed `RetryBuilder` with sensible defaults, comprehensive test coverage.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 8.1 | ⬜ | **Safety** | `unsafe` in `Task::eval` — fragile `ptr::read` + `mem::forget` on `LazyCell` | [task.rs:20-25](libs/xpipe/src/task.rs#L20) |
| 8.2 | ⬜ | **Bug** | `Timeout` leaks threads that are never cancelled | [time.rs:41](libs/xpipe/src/op/time.rs#L41) |
| 8.3 | ⬜ | **Bug** | `ForkHandle` doesn't handle mutex poisoning from panicked threads | [fork.rs:30](libs/xpipe/src/op/fork.rs#L30) |
| 8.4 | ⬜ | **Naming** | `async` in `task!` macro actually means "OS thread spawn" | [lib.rs:22](libs/xpipe/src/lib.rs#L22) |
| 8.5 | ⬜ | **Naming** | `max_attempts` means "max retries" (off-by-one semantics) | [result.rs:57](libs/xpipe/src/op/result.rs#L57) |
| 8.6 | ⬜ | **Design** | `TryMap` hardcoded to `TaskError` — can't use custom error types | [try_map.rs:3](libs/xpipe/src/op/try_map.rs#L3) |
| 8.7 | ⬜ | **Design** | `Deref` on `Task` forces eager evaluation — printing triggers computation | [task.rs:28](libs/xpipe/src/task.rs#L28) |

### Details

#### 8.1 — Unsafe in `Task::eval`

```rust
pub fn eval(self) -> T {
    let ptr = LazyCell::force(&self.0) as *const T;
    let value = unsafe { std::ptr::read(ptr) };
    std::mem::forget(self);
    value
}
```

This performs `ptr::read` to move a value out of `LazyCell`, then `mem::forget` to prevent double-drop. Fragile — relies on `LazyCell` internal layout assumptions.

**Fix:** Replace with a safe alternative:

```rust
enum TaskInner<T> {
    Lazy(Box<dyn FnOnce() -> T + Send>),
    Ready(T),
}

impl<T> Task<T> {
    pub fn eval(mut self) -> T {
        match self.inner.take().unwrap() {
            TaskInner::Lazy(f) => f(),
            TaskInner::Ready(v) => v,
        }
    }
}
```

Zero `unsafe`, same performance. Tradeoff: loses `Deref` auto-evaluation (which is itself a footgun).

#### 8.2 — Thread leaks in `Timeout`

When a timeout fires, the spawned thread continues running forever. The `tx.send(result)` silently fails (because `rx` is dropped) but the computation keeps wasting resources.

**Fix:** Document the limitation. For CPU-bound work, this is essentially a resource leak.

#### 8.3 — Mutex poisoning in `ForkHandle`

If the spawned thread panics during `task.eval()`, the mutex is poisoned. `lock.lock().unwrap()` then panics with "poisoned lock" instead of propagating the original error.

**Fix:** Use `std::panic::catch_unwind` in the spawned thread. Store `Result<T, Box<dyn Any + Send>>` in shared state.

#### 8.5 — `max_attempts` off-by-one

With `max_attempts = 2`, you get 3 total attempts (1 initial + 2 retries). The test confirms this: `.attempts(2)` results in `counter == 3`.

**Fix:** Rename to `max_retries` or change semantics to match the name.

### Missing

- `#[must_use]` on `Task<T>` and `ForkHandle<T>`
- `FlatMap` / `and_then` combinator
- `Zip` / `join` for combining multiple tasks
- `map_err` for Result pipelines
- Zero doc comments (`///`) anywhere in the crate
- `rust-version = "1.80"` in Cargo.toml (uses `LazyCell`)

---

## Cross-Cutting Themes

### C.1 — No generics support in derive macros

~~[xval-derive](libs/xval-derive/src/lib.rs)~~ ✅ Fixed — now uses `input.generics.split_for_impl()` with integration tests covering generic structs and enums.

[xsch-derive](libs/xsch-derive/src/lib.rs) still breaks on generic types and needs the same treatment (issue 6.2).

### C.2 — Panicking APIs where `Result`/`Option` is expected

- `Scope::render()` — [scope.rs:64](libs/xtera/src/scope.rs#L64)
- `From<&str> for Path` — [path.rs:64](libs/xpath/src/path.rs#L64)
- `ForkHandle` mutex poisoning — [fork.rs:30](libs/xpipe/src/op/fork.rs#L30)

### C.3 — Missing standard trait implementations

Across multiple crates: `FromStr`, `IntoIterator`, `TryFrom`, `Hash`.

### C.4 — Dead code

- `Phase` enum — [phase.rs](libs/xsch/src/phase.rs)
- `Validate` trait — [lib.rs:60](libs/xsch/src/lib.rs#L60)
- `ForNode.track` — [for_node.rs:10](libs/xtera/src/ast/node/for_node.rs#L10)
- `Node::Block` variant — [node/mod.rs](libs/xtera/src/ast/node/mod.rs)

### C.5 — No crate-level documentation

None of the 8 crates have `//!` doc comments or usage examples at the crate root.

### C.6 — Test coverage

Most crates have good unit tests, though some public methods (especially in xpath and the derive crates) lack coverage. No crate uses `trybuild` for compile-fail tests on derive macros.

---

## Priority Summary

Remaining items:

1. ⬜ **Move type checking before rules** in xsch (5.1) — rules can panic on wrong types
2. ⬜ **Eliminate `unsafe`** in xpipe's `Task::eval` (8.1)
3. ⬜ **Handle generics** in xsch-derive (6.2)
4. ⬜ **Fix panicking `From<&str>`** in xpath (7.1)
5. ⬜ **Fix `Scope::render` panic** and add `@include` recursion guard in xtera (3.1, 3.2)
6. ⬜ **Guard `Equals`/`Options`/`Pattern`** against null/wrong types in xsch (5.2, 5.3)
7. ⬜ **Handle `Option<T>`** in xsch-derive (6.1)
