# xpath

Lightweight path representation for navigating hierarchical data. Used throughout the workspace to address nested values and pinpoint validation error locations.

A path is a `/`-separated sequence of string keys and numeric indices. Numeric segments are automatically parsed as array indices, everything else as object keys.

## Parsing Paths

```rust
use xpath::Path;

let path = Path::parse("users/0/name").unwrap();
assert_eq!(path.len(), 3);
assert_eq!(path[0], "users");
assert_eq!(path[1], 0usize);
assert_eq!(path[2], "name");

// Display round-trips
assert_eq!(path.to_string(), "users/0/name");
```

Empty paths are valid. Empty segments, leading slashes, and trailing slashes are not:

```rust
assert!(Path::parse("").unwrap().is_empty());
assert!(Path::parse("a//b").is_err());
assert!(Path::parse("/a").is_err());
```

## Building Paths

```rust
use xpath::{Path, Ident};

let mut path = Path::default();
path.push(Ident::key("users"));
path.push(Ident::index(0));
path.push(Ident::key("name"));

// child appends a segment
let with_email = path.child(Ident::key("email"));

// peer replaces the last segment
let sibling = path.peer(Ident::key("age")); // users/0/age
```

## Converting from Strings

```rust
let path: xpath::Path = "config/db/host".into();
```

## Features

| Feature | Description |
|---------|-------------|
| `serde` | `Path` serializes as a string, `Ident` as a string or number |
