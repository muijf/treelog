<div align="center">

![TreeLog Banner](images/banner.png)

[![CI](https://github.com/muijf/treelog/workflows/CI/badge.svg)](https://github.com/muijf/treelog/actions)
[![crates.io](https://img.shields.io/crates/v/treelog.svg)](https://crates.io/crates/treelog)
[![docs.rs](https://docs.rs/treelog/badge.svg)](https://docs.rs/treelog)
[![license](https://img.shields.io/crates/l/treelog.svg)](https://github.com/muijf/treelog/blob/main/LICENSE)

**A customizable tree rendering library for Rust**

*Provides low-level and high-level APIs for rendering hierarchical data structures, similar to `tree`, `npm ls`, or `cargo tree`.*

[Installation](#installation) • [Quick Start](#quick-start) • [Documentation](https://docs.rs/treelog) • [Examples](examples/)

</div>

---

## Features

- **Multiple Styles** - Unicode, ASCII, Box drawing, and custom styles
- **Builder API** - Fluent interface for easy tree construction
- **Iterator Support** - Stream large trees without materializing
- **Custom Formatters** - Format nodes and leaves to your needs
- **Optimized** - Pre-computed prefixes and efficient rendering
- **Zero Dependencies** - Lightweight and fast

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
treelog = "0.0.3"
```

## Quick Start

```rust
use treelog::{Tree, renderer::write_tree};

let tree = Tree::Node("root".to_string(), vec![
    Tree::Leaf(vec!["item1".to_string()]),
    Tree::Node("sub".to_string(), vec![
        Tree::Leaf(vec!["subitem1".to_string()]),
        Tree::Leaf(vec!["subitem2".to_string()]),
    ]),
]);

let mut output = String::new();
write_tree(&mut output, &tree).unwrap();
println!("{}", output);
```

**Output:**
```text
root
├─ item1
└─ sub
   ├─ subitem1
   └─ subitem2
```

## Usage

### Basic

```rust
use treelog::{Tree, renderer::write_tree};

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
let mut output = String::new();
write_tree(&mut output, &tree).unwrap();
// root
//  └─ item
```

### Builder API

```rust
use treelog::builder::TreeBuilder;

let mut builder = TreeBuilder::new();
builder.node("root").leaf("item1").node("sub").leaf("subitem1").leaf("subitem2").end().leaf("item2");
let tree = builder.build();
println!("{}", tree.render_to_string());
```

### Low-Level API

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Leaf(vec!["line1".to_string(), "line2".to_string()]),
    Tree::Node("sub".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]),
]);
println!("{}", tree.render_to_string());
```

### Iterator API

```rust
use treelog::{Tree, TreeIteratorExt};

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

// Stream lines
for line in TreeIteratorExt::lines(&tree) {
    println!("{} {}", line.prefix, line.content);
}

// Or collect all at once
let lines: Vec<String> = tree.to_lines();
```

### Customization

**Styles**:

```rust
use treelog::{Tree, TreeStyle, RenderConfig, StyleConfig};

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

// Predefined styles
tree.render_to_string_with_config(&RenderConfig::default().with_style(TreeStyle::Ascii));

// Custom style
let style = StyleConfig::custom("├─", "└─", "│ ", "   ");
tree.render_to_string_with_config(&RenderConfig::default().with_style(style));
```

**Formatters** (requires `formatters` feature):

```rust
use treelog::{Tree, RenderConfig};

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
let config = RenderConfig::default()
    .with_node_formatter(|label| format!("[{}]", label))
    .with_leaf_formatter(|line| format!("- {}", line));
tree.render_to_string_with_config(&config);
```

### Streaming Large Trees

```rust,no_run
use treelog::{Tree, TreeIteratorExt};
use std::io::Write;

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
let mut stdout = std::io::stdout();
for line in TreeIteratorExt::lines(&tree) {
    writeln!(stdout, "{}{}", line.prefix, line.content).unwrap();
}
```

## Performance

- **Pre-computed prefixes** - Efficient string buffer capacity estimation
- **Iterator API** - Stream large trees without materializing the entire structure
- **Stack-based traversal** - Memory-efficient tree walking

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Development

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

**Real-time feedback**: This project includes VS Code/Cursor settings (`.vscode/settings.json`) that configure rust-analyzer to provide real-time feedback:
- **Formatting**: Automatically formats on save (matches `cargo fmt`)
- **Clippy**: Shows clippy warnings/errors as you type (matches pre-commit hook settings)
- **Tests**: Displays test failures in the editor

Git hooks available via [pre-commit](https://pre-commit.com/) (see `.pre-commit-config.yaml`).

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
