<div align="center">

![TreeLog Banner](images/banner.png)

[![CI](https://github.com/muijf/treelog/workflows/CI/badge.svg)](https://github.com/muijf/treelog/actions)
[![crates.io](https://img.shields.io/crates/v/treelog.svg)](https://crates.io/crates/treelog)
[![docs.rs](https://docs.rs/treelog/badge.svg)](https://docs.rs/treelog)
[![license](https://img.shields.io/crates/l/treelog.svg)](https://github.com/muijf/treelog/blob/main/LICENSE)

**A customizable tree rendering library for Rust**

*Provides low-level and high-level APIs for rendering hierarchical data structures, similar to `tree`, `npm ls`, or `cargo tree`.*

[Installation](#installation) • [Quick Start](#quick-start) • [Documentation](https://docs.rs/treelog) • [Examples](#examples)

</div>

---

## Features

### Core Features
- **Multiple Styles** - Unicode, ASCII, Box drawing, and custom styles
- **Macro DSL** - Declarative syntax for easy tree construction
- **Builder API** - Fluent interface for easy tree construction
- **Iterator Support** - Stream large trees without materializing
- **Custom Formatters** - Format nodes and leaves to your needs
- **Optimized** - Pre-computed prefixes and efficient rendering
- **Zero Dependencies** - Lightweight and fast

### Analysis & Statistics
- **Tree Statistics** - Get depth, width, node/leaf counts, and more
- **Tree Traversal** - Pre-order, post-order, level-order, and filtered iterators
- **Tree Search** - Find nodes/leaves by label or content, get paths

### Transformation & Manipulation
- **Tree Transformation** - Map, filter, and prune trees functionally
- **Tree Sorting** - Sort children by label, depth, or custom comparison
- **Tree Merging** - Merge trees with different strategies (replace, append, merge-by-label)

### Utilities
- **Path Utilities** - Navigate trees by path, flatten to lists
- **Tree Comparison** - Compare structure, compute diffs, check subtrees
- **Export Formats** - Export to HTML, SVG, and Graphviz DOT format
- **Tree Validation** - Validate tree structure

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

### Macro DSL

The `tree!` macro provides a clean, declarative syntax for constructing trees (requires `macro` feature):

```rust
use treelog::tree;

let tree = tree! {
    root {
        "item1",
        "item2",
        sub {
            "subitem1",
            "subitem2"
        }
    }
};
println!("{}", tree.render_to_string());
```

The macro supports:
- **Nodes**: `identifier { ... }` or `"string" { ... }`
- **Leaves**: `"string"` or bare identifiers (converted to strings)
- **Nested structures**: Arbitrary nesting depth
- **Comma-separated**: Children separated by commas (trailing comma optional)

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

## Advanced Features

<details>
<summary><b>Tree Statistics</b> - Get detailed metrics about your tree</summary>

Get detailed statistics about your tree:

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Node("src".to_string(), vec![Tree::Leaf(vec!["main.rs".to_string()])]),
    Tree::Leaf(vec!["README.md".to_string()]),
]);

// Individual statistics
println!("Depth: {}", tree.depth());
println!("Width: {}", tree.width());
println!("Nodes: {}", tree.node_count());
println!("Leaves: {}", tree.leaf_count());

// Or get all at once
let stats = tree.stats();
println!("{:#?}", stats);
```

</details>

<details>
<summary><b>Tree Traversal</b> - Iterate over trees in different orders (requires <code>traversal</code> feature)</summary>

Iterate over trees in different orders:

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Leaf(vec!["item".to_string()])
]);

// Pre-order: root, then children
for node in tree.pre_order() {
    // Process node
}

// Post-order: children, then root
for node in tree.post_order() {
    // Process node
}

// Level-order: breadth-first
for node in tree.level_order() {
    // Process node
}

// Iterate only over nodes or leaves
for node in tree.nodes() {
    println!("Node: {}", node.label().unwrap());
}

for leaf in tree.leaves() {
    println!("Leaf: {:?}", leaf.lines());
}
```

</details>

<details>
<summary><b>Tree Search</b> - Find nodes and leaves in your tree</summary>

Find nodes and leaves in your tree:

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Node("src".to_string(), vec![Tree::Leaf(vec!["main.rs".to_string()])]),
]);

// Find first node with label
if let Some(node) = tree.find_node("src") {
    println!("Found: {}", node.label().unwrap());
}

// Find all nodes with label
let all_nodes = tree.find_all_nodes("src");

// Find leaf containing content
if let Some(leaf) = tree.find_leaf("main.rs") {
    println!("Found leaf");
}

// Check if tree contains label/content
if tree.contains("src") {
    println!("Tree contains 'src'");
}

// Get path to a node
if let Some(path) = tree.path_to("main.rs") {
    println!("Path: {:?}", path); // [0, 0]
}
```

</details>

<details>
<summary><b>Tree Transformation</b> - Transform trees functionally (requires <code>transform</code> feature)</summary>

Transform trees functionally:

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Node("src".to_string(), vec![Tree::Leaf(vec!["main.rs".to_string()])]),
]);

// Map node labels
let transformed = tree.map_nodes(|label| format!("[{label}]"));

// Map leaf lines
let transformed = tree.map_leaves(|line| format!("- {line}"));

// Filter tree
let filtered = tree.filter(|t| {
    match t {
        Tree::Leaf(lines) => lines.iter().any(|l| l.contains("main")),
        Tree::Node(_, _) => true,
    }
});

// Prune tree (inverse of filter)
let pruned = tree.prune(|t| {
    match t {
        Tree::Leaf(lines) => lines.iter().any(|l| l.contains("README")),
        Tree::Node(_, _) => false,
    }
});
```

</details>

<details>
<summary><b>Tree Sorting</b> - Sort tree children</summary>

Sort tree children:

```rust
use treelog::Tree;

let mut tree = Tree::Node("root".to_string(), vec![
    Tree::Node("zebra".to_string(), vec![]),
    Tree::Node("apple".to_string(), vec![]),
]);

// Sort by label alphabetically
tree.sort_by_label();

// Sort by depth
tree.sort_by_depth(true); // deepest first

// Custom sort
let mut compare = |a: &Tree, b: &Tree| {
    // Your comparison logic
    std::cmp::Ordering::Equal
};
tree.sort_children(&mut compare);
```

Navigate and manipulate trees by path:

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Node("src".to_string(), vec![Tree::Leaf(vec!["main.rs".to_string()])]),
]);

// Get path to a node
let child = &tree.children().unwrap()[0];
if let Some(path) = tree.get_path(child) {
    println!("Path: {:?}", path); // [0]
}

// Get node by path
if let Some(node) = tree.get_by_path(&[0]) {
    println!("Node: {}", node.label().unwrap());
}

// Modify node by path
let mut tree = tree.clone();
if let Some(Tree::Node(label, _)) = tree.get_by_path_mut(&[0]) {
    *label = "source".to_string();
}

// Flatten tree to list
let flattened = tree.flatten();
for entry in flattened {
    println!("Path: {:?}, Content: {}", entry.path, entry.content);
}
```

</details>

<details>
<summary><b>Tree Comparison</b> - Compare trees and find differences (requires <code>comparison</code> feature)</summary>

Compare trees and find differences:

```rust
use treelog::Tree;

let tree1 = Tree::Node("root".to_string(), vec![
    Tree::Leaf(vec!["a".to_string()])
]);
let tree2 = Tree::Node("root".to_string(), vec![
    Tree::Leaf(vec!["b".to_string()])
]);

// Compare structure (ignoring content)
if tree1.eq_structure(&tree2) {
    println!("Same structure!");
}

// Compute differences
let diffs = tree1.diff(&tree2);
for diff in diffs {
    match diff {
        treelog::comparison::TreeDiff::OnlyInFirst { path, content } => {
            println!("Only in first: {:?} - {}", path, content);
        }
        treelog::comparison::TreeDiff::OnlyInSecond { path, content } => {
            println!("Only in second: {:?} - {}", path, content);
        }
        treelog::comparison::TreeDiff::DifferentContent { path, first, second } => {
            println!("Different at {:?}: '{}' vs '{}'", path, first, second);
        }
    }
}

// Check if subtree
let subtree = Tree::Leaf(vec!["a".to_string()]);
if subtree.is_subtree_of(&tree1) {
    println!("Is a subtree!");
}
```

</details>

<details>
<summary><b>Tree Merging</b> - Merge trees with different strategies (requires <code>merge</code> feature)</summary>

Merge trees with different strategies:

```rust
use treelog::{Tree, merge::MergeStrategy};

let tree1 = Tree::Node("root".to_string(), vec![
    Tree::Node("src".to_string(), vec![Tree::Leaf(vec!["main.rs".to_string()])]),
]);
let tree2 = Tree::Node("root".to_string(), vec![
    Tree::Node("src".to_string(), vec![Tree::Leaf(vec!["lib.rs".to_string()])]),
]);

// Replace: replace first tree with second
let merged = tree1.merge(tree2.clone(), MergeStrategy::Replace);

// Append: append all children
let merged = tree1.merge(tree2.clone(), MergeStrategy::Append);

// MergeByLabel: merge nodes with matching labels
let merged = tree1.merge(tree2, MergeStrategy::MergeByLabel);
```

</details>

<details>
<summary><b>Export Formats</b> - Export trees to HTML, SVG, and DOT (requires <code>export</code> feature)</summary>

Export trees to various formats:

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Leaf(vec!["item".to_string()])
]);

// Export to HTML (with collapsible nodes)
let html = tree.to_html();
std::fs::write("tree.html", html).unwrap();

// Export to SVG
let svg = tree.to_svg();
std::fs::write("tree.svg", svg).unwrap();

// Export to Graphviz DOT format
let dot = tree.to_dot();
std::fs::write("tree.dot", dot).unwrap();
// Render with: dot -Tpng tree.dot -o tree.png
```

</details>

## Feature Flags

Most advanced features are behind feature flags to keep the core library lightweight:

```toml
[dependencies]
treelog = { version = "0.0.3", features = ["traversal", "transform", "path", "comparison", "merge", "export"] }
```

Available features:
- `traversal` - Tree traversal iterators (pre-order, post-order, level-order)
- `transform` - Tree transformation operations (map, filter, prune)
- `path` - Tree path utilities (get by path, flatten)
- `comparison` - Tree comparison and diff operations
- `merge` - Tree merging with different strategies
- `export` - Export to HTML, SVG, and DOT formats
- `builder` - Builder API for constructing trees
- `iterator` - Iterator API for streaming trees
- `macro` - Macro DSL for tree construction
- `formatters` - Custom formatters for nodes and leaves
- `color` - Color output support

Use `all` feature to enable everything:
```toml
treelog = { version = "0.0.3", features = ["all"] }
```

## Performance

- **Pre-computed prefixes** - Efficient string buffer capacity estimation
- **Iterator API** - Stream large trees without materializing the entire structure
- **Stack-based traversal** - Memory-efficient tree walking
- **Zero-copy operations** - Most operations work with references where possible

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

## Examples

The repository includes comprehensive examples demonstrating all features:

- **[`basic`](examples/basic.rs)** - Basic tree construction and rendering
- **[`builder`](examples/builder.rs)** - Using the builder API
- **[`iterator`](examples/iterator.rs)** - Streaming large trees
- **[`macro`](examples/macro.rs)** - Using the macro DSL
- **[`customization`](examples/customization.rs)** - Custom styles and formatters
- **[`complex`](examples/complex.rs)** - Complex tree structures
- **[`file_tree`](examples/file_tree.rs)** - File system tree example
- **[`statistics`](examples/statistics.rs)** - Tree statistics and analysis
- **[`traversal`](examples/traversal.rs)** - Tree traversal iterators
- **[`search`](examples/search.rs)** - Tree search and query operations
- **[`transform`](examples/transform.rs)** - Tree transformation operations
- **[`sorting`](examples/sorting.rs)** - Tree sorting operations
- **[`path`](examples/path.rs)** - Tree path utilities
- **[`comparison`](examples/comparison.rs)** - Tree comparison and diff
- **[`merge`](examples/merge.rs)** - Tree merging strategies
- **[`export`](examples/export.rs)** - Export to HTML, SVG, and DOT formats

Run any example with:
```bash
cargo run --example <name> --all-features
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
