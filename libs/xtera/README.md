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

### Collections

```html
{{ setCols([1, 2, 3]) }}
{{ setConfig({ theme: 'dark', count: 5 }) }}
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
| Array literals | `[1, 2, 3]`, `[]` |
| Object literals | `{ a: 1, b: 'two' }`, `{}` |

## Usage

```rust
use xtera::ast::NodeKind;
use xtera::parse;

let template = parse::parse("Hello {{ name | uppercase }}!").unwrap();

for node in &template.nodes {
    match &node.kind {
        NodeKind::Text(text) => {
            println!("text: {text}");
        }
        NodeKind::Interpolation(expr) => {
            println!("expr: {expr:?}");
        }
        NodeKind::If(block) => {
            println!("if: {} branches", block.branches.len());
        }
        NodeKind::For(block) => {
            println!("for: {} of {:?}", block.binding, block.iterable);
        }
        NodeKind::Switch(block) => {
            println!("switch: {} cases", block.cases.len());
        }
    }
}
```

## Architecture

```
ast/                AST node types (the parsed template tree)
├── node.rs         Template, Node, NodeKind, IfBlock, ForBlock, SwitchBlock
├── expr.rs         Expr, ExprKind, Literal
├── op.rs           BinaryOp, UnaryOp
└── span.rs         Source location tracking

parse/              Parsing machinery (template string -> AST)
├── token.rs        Token enum via #[derive(Logos)]
├── lexer.rs        Dual-mode lexer (text mode + expression mode)
├── parser.rs       Recursive descent parser with Pratt precedence
└── error.rs        ParseError type
```
