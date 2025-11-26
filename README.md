# TreeLog

[![CI](https://github.com/muijf/treelog/workflows/CI/badge.svg)](https://github.com/muijf/treelog/actions)
[![crates.io](https://img.shields.io/crates/v/treelog.svg)](https://crates.io/crates/treelog)
[![docs.rs](https://docs.rs/treelog/badge.svg)](https://docs.rs/treelog)
[![license](https://img.shields.io/crates/l/treelog.svg)](https://github.com/muijf/treelog/blob/main/LICENSE)

A highly customizable, optimized, and modular tree rendering library for Rust.

This library provides both low-level and high-level APIs for rendering hierarchical data structures as trees, similar to the output of tools like `tree`, `npm ls`, or `cargo tree`.

## Features

- **Multiple Styles**: Unicode, ASCII, Box drawing, and custom character sets
- **Flexible API**: Both low-level and high-level builder APIs
- **Iterator Support**: Stream lines one at a time without materializing the entire tree
- **Customizable**: Per-node formatting, custom styles, and configuration options
- **Optimized**: Pre-computed prefixes, efficient string allocation
- **Well Documented**: Comprehensive documentation with examples

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
treelog = "0.1"
```

### Basic Usage

```rust
use treelog::{Tree, renderer::write_tree};

let tree = Tree::Node(
    "root".to_string(),
    vec![Tree::Leaf(vec!["item".to_string()])],
);

let mut output = String::new();
write_tree(&mut output, &tree).unwrap();
println!("{}", output);
```

Output:

```text
root
 └─ item
```

## API Overview

### High-Level Builder API

The builder API provides a fluent interface for constructing trees:

```rust
use treelog::builder::TreeBuilder;

let mut builder = TreeBuilder::new();
builder.node("root").leaf("item1").node("sub").leaf("subitem1").leaf("subitem2").end().leaf("item2");
let tree = builder.build();

println!("{}", tree.render_to_string());
```

Output:

```text
root
 ├─ item1
 ├─ sub
 │  ├─ subitem1
 │  └─ subitem2
 └─ item2
```

### Low-Level API

For more control, construct trees directly:

```rust
use treelog::Tree;

let l1 = Tree::Leaf(vec!["line1".to_string(), "line2".to_string()]);
let l2 = Tree::Leaf(vec!["only one line".to_string()]);
let n1 = Tree::Node("node 1".to_string(), vec![l1.clone(), l2.clone()]);
let n2 = Tree::Node("node 2".to_string(), vec![l2.clone(), l1.clone()]);
let n3 = Tree::Node("node 3".to_string(), vec![n1.clone(), l1.clone(), l2.clone()]);
let n4 = Tree::Node("node 4".to_string(), vec![n1, n2, n3]);

println!("{}", n4.render_to_string());
```

### Iterator API

Get lines one at a time for streaming or custom processing:

```rust
use treelog::{Tree, TreeIteratorExt};

let tree = Tree::Node(
    "root".to_string(),
    vec![Tree::Leaf(vec!["item".to_string()])],
);

for line in TreeIteratorExt::lines(&tree) {
    println!("Prefix: '{}', Content: '{}'", line.prefix, line.content);
}
```

Or collect all lines at once:

```rust
use treelog::{Tree, TreeIteratorExt};

let tree = Tree::Node(
    "root".to_string(),
    vec![Tree::Leaf(vec!["item".to_string()])],
);
let lines: Vec<String> = tree.to_lines();
for line in lines {
    println!("{}", line);
}
```

### Customization

#### Different Styles

```rust
use treelog::{Tree, TreeStyle, RenderConfig};

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

// Unicode style (default)
let output1 = tree.render_to_string_with_config(
    &RenderConfig::default().with_style(TreeStyle::Unicode)
);

// ASCII style
let output2 = tree.render_to_string_with_config(
    &RenderConfig::default().with_style(TreeStyle::Ascii)
);

// Custom style
use treelog::StyleConfig;
let custom_style = StyleConfig::custom(">", "<", "|", " ");
let output3 = tree.render_to_string_with_config(
    &RenderConfig::default().with_style(custom_style)
);
```

#### Custom Formatters

```rust
use treelog::{Tree, RenderConfig};

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

let config = RenderConfig::default()
    .with_node_formatter(|label| format!("[{}]", label))
    .with_leaf_formatter(|line| format!("- {}", line));

let output = tree.render_to_string_with_config(&config);
```

#### Custom Style Configuration

```rust
use treelog::{StyleConfig, RenderConfig};

let style = StyleConfig::custom("├─", "└─", "│ ", "   ");
let config = RenderConfig::default().with_style(style);
```

## Advanced Examples

### Complex Tree with Multiple Lines

```rust
use treelog::Tree;

let l1 = Tree::Leaf(vec![
    "line1".to_string(),
    "line2".to_string(),
    "line3".to_string(),
    "line4".to_string(),
]);
let l2 = Tree::Leaf(vec!["only one line".to_string()]);
let n1 = Tree::Node("node 1".to_string(), vec![l1.clone(), l2.clone()]);
let n2 = Tree::Node("node 2".to_string(), vec![l2.clone(), l1.clone(), l2.clone()]);
let n3 = Tree::Node("node 3".to_string(), vec![n1.clone(), l1.clone(), l2.clone()]);
let n4 = Tree::Node("node 4".to_string(), vec![n1, n2, n3]);

println!("{}", n4.render_to_string());
```

Output:

```text
node 4
├─ node 1
│  ├─ line1
│  │  line2
│  │  line3
│  │  line4
│  └─ only one line
├─ node 2
│  ├─ only one line
│  ├─ line1
│  │  line2
│  │  line3
│  │  line4
│  └─ only one line
└─ node 3
   ├─ node 1
   │  ├─ line1
   │  │  line2
   │  │  line3
   │  │  line4
   │  └─ only one line
   ├─ line1
   │  line2
   │  line3
   │  line4
   └─ only one line
```

### Streaming Large Trees

For very large trees, use the iterator API to avoid materializing the entire string:

```rust,no_run
use treelog::{Tree, TreeIteratorExt};
use std::io::{self, Write};

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

let mut stdout = io::stdout();
for line in TreeIteratorExt::lines(&tree) {
    writeln!(stdout, "{}{}", line.prefix, line.content).unwrap();
}
```

## Performance

The library is optimized for performance:

- **Pre-computed prefixes**: Tree structure prefixes are computed efficiently
- **Capacity estimation**: String buffers are pre-allocated with estimated capacity
- **Zero-copy where possible**: Iterator API avoids unnecessary allocations
- **Efficient traversal**: Stack-based iteration minimizes overhead

## License

MIT

## Development

### Pre-commit Hooks

This project includes git hooks to ensure code quality. Install them with:

```bash
./scripts/install-hooks.sh
```

This will set up hooks that run:
- **pre-commit**: `cargo fmt` and `cargo clippy` checks
- **pre-push**: `cargo test` to ensure all tests pass

You can also use the [pre-commit](https://pre-commit.com/) framework (see `.pre-commit-config.yaml`):

```bash
pip install pre-commit
pre-commit install
```

### Manual Checks

Before committing, you can manually run:

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

