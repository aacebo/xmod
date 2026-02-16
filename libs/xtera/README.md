# XTera

Typed Expression Rendering Architecture. An Angular-style template parser built on [logos](https://github.com/maciejhirsz/logos).

## Syntax

### Interpolation

```html
{{ expression }}
{{ user.name }}
{{ value | uppercase }}
{{ value | slice:0:5 }}
```

### Control Flow

**Conditionals**

```html
@if (isVisible) {
  <p>Visible</p>
} @else @if (isFallback) {
  <p>Fallback</p>
} @else {
  <p>Hidden</p>
}
```

**Iteration**

```html
@for (item of items; track item.id) {
  <li>{{ item.name }}</li>
}
```

**Switch**

```html
@switch (color) {
  @case ('red') {
    <span>Red</span>
  }
  @case ('blue') {
    <span>Blue</span>
  }
  @default {
    <span>Unknown</span>
  }
}
```

## Expressions

| Feature | Example |
|---------|---------|
| Identifiers | `name`, `_count`, `$ref` |
| Member access | `user.name` |
| Index access | `items[0]` |
| Function calls | `greet('world')` |
| Method calls | `list.contains(item)` |
| Pipes | `value \| uppercase`, `value \| slice:0:5` |
| Binary operators | `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `\|\|` |
| Unary operators | `!`, `-` |
| Grouping | `(a + b) * c` |
| Literals | `42`, `3.14`, `'hello'`, `"world"`, `true`, `false`, `null` |

## Usage

```rust
use xtera::parse;

let template = parse::parse("Hello {{ name | uppercase }}!").unwrap();

for node in &template.nodes {
    match &node.kind {
        parse::NodeKind::Text(text) => {
            println!("text: {text}");
        }
        parse::NodeKind::Interpolation(expr) => {
            println!("expr: {expr:?}");
        }
        parse::NodeKind::If(block) => {
            println!("if: {} branches", block.branches.len());
        }
        parse::NodeKind::For(block) => {
            println!("for: {} of {:?}", block.binding, block.iterable);
        }
        parse::NodeKind::Switch(block) => {
            println!("switch: {} cases", block.cases.len());
        }
    }
}
```

## Architecture

```
parse/
├── token.rs    Token enum via #[derive(Logos)]
├── lexer.rs    Dual-mode lexer (text mode + expression mode)
├── parser.rs   Recursive descent parser with Pratt precedence
├── node.rs     Template AST (Text, Interpolation, If, For, Switch)
├── expr.rs     Expression AST (Ident, Member, Call, Pipe, Binary, ...)
├── op.rs       BinaryOp / UnaryOp enums
├── span.rs     Source location tracking
└── error.rs    ParseError type
```
