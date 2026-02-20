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

## 7. xpath — Path Navigation

**Strengths:** Clean, focused design. Correct parsing. Good serde integration. Solid test coverage for parse/display.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 7.1 | ⬜ | **Design** | `From<&str>` panics on invalid input — violates infallible `From` contract | [path.rs:64](libs/xpath/src/path.rs#L64) |
| 7.2 | ⬜ | **Testing** | `push`, `pop`, `child`, `peer`, `iter` have zero test coverage | — |
| 7.3 | ⬜ | **Design** | No `FromStr`/`IntoIterator`/`TryFrom` standard trait impls | — |
| 7.4 | ⬜ | **Design** | Numeric keys impossible — `"42"` always becomes `Index(42)` | [ident.rs:30](libs/xpath/src/ident.rs#L30) |

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
- `rust-version = "1.80"` in Cargo.toml (uses `LazyCell`)

---

## Cross-Cutting Themes

### C.2 — Panicking APIs where `Result`/`Option` is expected

- `From<&str> for Path` — [path.rs:64](libs/xpath/src/path.rs#L64)
- `ForkHandle` mutex poisoning — [fork.rs:30](libs/xpipe/src/op/fork.rs#L30)

### C.3 — Missing standard trait implementations

Across multiple crates: `FromStr`, `IntoIterator`, `TryFrom`, `Hash`.

### C.4 — Dead code
- `ForNode.track` — [for_node.rs:10](libs/xtera/src/ast/node/for_node.rs#L10)

---

## Priority Summary

Remaining items:

1. ⬜ **Eliminate `unsafe`** in xpipe's `Task::eval` (8.1)
2. ⬜ **Fix panicking `From<&str>`** in xpath (7.1)
