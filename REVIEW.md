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

Overall the codebase is clean, well-organized, and shows strong Rust fundamentals. Below are the remaining findings.

---

## 8. xpipe — Lazy Pipeline Library

**Strengths:** Clean extension-trait pattern (`.map()`, `.filter()`, etc.), good operator set, well-designed `RetryBuilder` with sensible defaults, comprehensive test coverage.

### Issues

| # | Status | Severity | Issue | Location |
|---|--------|----------|-------|----------|
| 8.2 | ⬜ | **Bug** | `Timeout` leaks threads that are never cancelled | [time.rs:41](libs/xpipe/src/op/time.rs#L41) |
| 8.3 | ⬜ | **Bug** | `ForkHandle` doesn't handle mutex poisoning from panicked threads | [fork.rs:30](libs/xpipe/src/op/fork.rs#L30) |

### Details

#### 8.2 — Thread leaks in `Timeout`

When a timeout fires, the spawned thread continues running forever. The `tx.send(result)` silently fails (because `rx` is dropped) but the computation keeps wasting resources.

**Fix:** Document the limitation. For CPU-bound work, this is essentially a resource leak.

#### 8.3 — Mutex poisoning in `ForkHandle`

If the spawned thread panics during `task.eval()`, the mutex is poisoned. `lock.lock().unwrap()` then panics with "poisoned lock" instead of propagating the original error.

**Fix:** Use `std::panic::catch_unwind` in the spawned thread. Store `Result<T, Box<dyn Any + Send>>` in shared state.

### Missing

- `#[must_use]` on `Task<T>` and `ForkHandle<T>`
- `FlatMap` / `and_then` combinator
- `Zip` / `join` for combining multiple tasks
- `map_err` for Result pipelines
- `rust-version = "1.80"` in Cargo.toml (uses `LazyCell`)

---

## Cross-Cutting Themes

### C.2 — Panicking APIs where `Result`/`Option` is expected

- `ForkHandle` mutex poisoning — [fork.rs:30](libs/xpipe/src/op/fork.rs#L30)

### C.4 — Dead code

- `ForNode.track` — [for_node.rs:10](libs/xtera/src/ast/node/for_node.rs#L10)

---

## Priority Summary

Remaining items:

1. ⬜ **Fix thread leak in `Timeout`** in xpipe (8.2)
2. ⬜ **Handle mutex poisoning in `ForkHandle`** in xpipe (8.3)
