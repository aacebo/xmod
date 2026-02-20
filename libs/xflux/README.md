# xflux

Action/trigger workflow framework. Define async actions that run logic, wire them to triggers that fire on events, and compose them into declarative workflows with schema-validated inputs and templated URLs.

## Actions

An action receives an execution context and returns a value:

```rust
use async_trait::async_trait;
use xflux::{Action, Context};
use xval::ext::StructExt;

struct Greet;

#[async_trait]
impl Action for Greet {
    async fn exec(&self, ctx: &mut Context) -> xok::Result<xval::Value> {
        let name = ctx.input.as_struct().get("name").unwrap().to_value();
        Ok(xval::valueof!((format!("hello {}", name.as_str()))))
    }
}
```

## Triggers

Triggers subscribe to actions and invoke them when an event occurs:

```rust
use xflux::{Trigger, Action};

struct OnStartup { actions: Vec<Box<dyn Action>> }

impl Trigger for OnStartup {
    fn subscribe(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }
}
```

## Context

The execution context carries the input value, a key-value data store for sharing state between actions, and a start timestamp:

```rust
use xflux::Context;

let mut ctx = Context::new(xval::valueof!({ "user": "alice" }));

ctx.set("token", "abc123");
assert_eq!(ctx.get("token").unwrap().as_str(), "abc123");
assert!(ctx.exists("token"));

let elapsed = ctx.elapse();
```

## Events

```rust
use xflux::Event;

let event = Event::new("user_created", xval::valueof!({ "id": 1_i32 }));
assert_eq!(event.name, "user_created");
```

## Workflow Definition (YAML)

Workflows compose actions declaratively. URLs are `xtera` templates, inputs are validated against `xsch` schemas:

```yaml
version: "0.0.1"
name: my-workflow
display_name: "My Workflow"
actions:
    - id: get_user
      type: http::get
      input:
        uri: "https://my.api.com/v1/users/me"
        headers:
          Authorization: "Bearer {{ $token }}"
      output: "$.data"

    - id: log_user
      type: log::info
      input: "user => {{ json($get_user) }}"
```

## Features

| Feature | Description |
|---------|-------------|
| `serde` | `Serialize`/`Deserialize` for action specs, context, and events |
