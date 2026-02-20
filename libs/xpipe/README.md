# xpipe

Lazy, composable pipelines for Rust. Build a chain of operations that only execute when you call `.eval()`. Supports threading, timeouts, retries, and all the standard `Result`/`Option` combinators.

## Quick Start

```rust
use xpipe::{task, op::*};

let result = task!(10)
    .map(|x| x * 2)
    .filter(|x| x > 5)
    .eval();

assert_eq!(result, Some(20));
```

## Creating Tasks

The `task!` macro is the primary way to create tasks:

```rust
task!(42)                        // from a value
task!("hello")                   // from a literal
task!(() => 1 + 2)               // lazy — runs on eval
task!(move () => captured * 2)   // lazy with move semantics
task!(async () => 42)            // spawns on a new thread
task!(async move () => x * 3)    // thread + move
```

Or construct directly:

```rust
use xpipe::Task;

let t = Task::from_static(42);
let t = Task::from_lazy(|| expensive_computation());
```

Nothing executes until `.eval()`:

```rust
let value = task!(() => heavy_work()).eval();
```

## Chaining Operations

```rust
use xpipe::{task, op::*};

let name = task!("  Alice  ")
    .map(|s| s.trim().to_string())
    .map(|s| s.to_lowercase())
    .eval();

assert_eq!(name, "alice");
```

## Error Handling

Work naturally with `Result` and `Option`:

```rust
use xpipe::{task, op::*};

// Result chains
let result = task!(Ok::<i32, String>(10))
    .and_then(|x| task!(Ok(x * 2)))
    .unwrap_or(0)
    .eval();

// Option chains
let found = task!(Some(42))
    .and_then(|x| task!(Some(x + 1)))
    .unwrap_or(0)
    .eval();
```

## Parallel Execution

Fork a task onto a separate thread:

```rust
use xpipe::{task, op::*};

let handle = task!(() => slow_computation()).fork();
// do other work...
let result = handle.eval(); // blocks until done
```

## Timeouts and Delays

```rust
use std::time::Duration;
use xpipe::{task, op::*};

let result = task!(() => slow_work())
    .timeout(Duration::from_secs(5))
    .eval(); // Ok(value) or Err(TimeoutError)

let delayed = task!(42)
    .delay(Duration::from_millis(100))
    .eval(); // waits, then returns 42
```

## Retries

```rust
use std::time::Duration;
use xpipe::{task, op::*};

let result = task!(() => flaky_call())
    .retry(|b| b.max_retries(3).delay(Duration::from_millis(100)))
    .eval();
```

## Combining Tasks

```rust
use xpipe::{task, op::*};

let pair = task!("hello")
    .zip(task!(42))
    .eval();

assert_eq!(pair, ("hello", 42));
```

## Reusable Routines

The `task!` macro can also create reusable functions:

```rust
let double = task!((x) => x * 2);
assert_eq!(double.eval(5), 10);
assert_eq!(double.eval(21), 42);
```

## Operator Reference

| Operator | Description |
|----------|-------------|
| `.map(f)` | Transform the value |
| `.try_map(f)` | Transform, returning `Result` |
| `.filter(pred)` | `Some(value)` if predicate passes, `None` otherwise |
| `.run(f)` | Side effect, pass value through |
| `.and_then(f)` | Flat map for `Result` or `Option` |
| `.zip(other)` | Combine two tasks into a tuple |
| `.fork()` | Spawn on a new thread |
| `.timeout(dur)` | Fail if evaluation exceeds duration |
| `.delay(dur)` | Sleep before evaluating |
| `.retry(config)` | Retry with exponential backoff |
| `.unwrap()` | Unwrap `Result`/`Option` or panic |
| `.expect(msg)` | Unwrap or panic with message |
| `.unwrap_or(val)` | Unwrap or use default |
| `.unwrap_or_else(f)` | Unwrap or compute default |
| `.ok()` | `Result<T,E>` to `Option<T>` |
| `.and(f)` | Validate a `Result` value |
| `.or(f)` | Fallback on `Result` error |
| `.map_err(f)` | Transform the error type |
