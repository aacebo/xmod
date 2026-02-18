# xflux — TODO

## Current State

The core abstractions exist (`Action`, `Trigger`, `Context`, `Event`, `FluxError`) but there is no executor, no routine definition format, no built-in actions, and no tests. xflux is a skeleton that defines the contract for a routine engine.

## Phase 1: Routine Executor

The missing centerpiece. A `Routine` or `Runner` type that takes a sequence of actions and executes them, threading the `Context` through each step.

- [ ] **Routine struct** — holds a name, a list of action steps, and optional trigger
- [ ] **Sequential execution** — run actions in order, passing context between them
- [ ] **Action results stored in context** — each action's output gets stored in `ctx` under its `id` so downstream actions can reference it (e.g. `$get_user`)
- [ ] **Error propagation** — if an action fails, halt the routine and surface the error with the failing step's id

## Phase 2: Schema Validation (xsch integration)

Actions should declare their input/output schemas so the engine can validate data at boundaries.

- [ ] **Input/output schemas on Action** — extend the trait or add metadata for `xsch::Schema` on inputs and outputs
- [ ] **Pre-execution validation** — validate the context/input against the action's input schema before calling `exec`
- [ ] **Post-execution validation** — validate the action's return value against its output schema

## Phase 3: Template Rendering (xtera integration)

The README shows `{{ $token }}` and `{{ json($get_user) }}` in action inputs. This requires integrating xtera to resolve expressions against the context before passing inputs to actions.

- [ ] **Render action inputs** — before executing an action, render its input config as a template with the current context as the scope
- [ ] **Context-to-Scope bridge** — convert `Context` data into an `xtera::Scope` for template evaluation

## Phase 4: Output Path Extraction (xpath integration)

The README shows `output: "$.data"` — an action's result should be narrowed via a path expression before storing in context.

- [ ] **Output path on action step** — optional `xpath::Path` that selects a sub-value from the action's result
- [ ] **Apply path after execution** — use `Value::get(&path)` to extract the selected portion

## Phase 5: Routine Definition Format

Parse routine definitions from a structured format (YAML, JSON, or a Rust builder API).

- [ ] **RoutineDef / Step structs** — serializable definition types: id, action type, input config, output path
- [ ] **Builder API** — programmatic routine construction
- [ ] **Optional: YAML/JSON parsing** — deserialize routine definitions (behind a `serde` feature)

## Phase 6: Trigger System

The `Trigger` trait is minimal. Flesh it out so triggers can start routines in response to events.

- [ ] **Event emission** — triggers should be able to fire events
- [ ] **Trigger lifecycle** — start/stop/subscribe patterns
- [ ] **Built-in triggers** — timer/cron, channel-based, webhook (behind features)

## Phase 7: Advanced Execution

- [ ] **Parallel steps** — run independent actions concurrently
- [ ] **Conditional steps** — skip actions based on an expression (reuse xtera for condition eval)
- [ ] **Retry / error recovery** — configurable retry policies per action
- [ ] **Timeouts** — per-action and per-routine timeouts (reuse xpipe's timeout operator pattern)

## Phase 8: Tests & Docs

- [ ] **Unit tests for Context**
- [ ] **Integration tests with mock actions**
- [ ] **End-to-end routine execution test**
- [ ] **README with usage examples**
