<div align="center">

![TreeLog Banner](images/banner.png)

[![CI](https://github.com/muijf/treelog/workflows/CI/badge.svg)](https://github.com/muijf/treelog/actions)
[![crates.io](https://img.shields.io/crates/v/treelog.svg)](https://crates.io/crates/treelog)
[![docs.rs](https://docs.rs/treelog/badge.svg)](https://docs.rs/treelog)
[![license](https://img.shields.io/crates/l/treelog.svg)](https://github.com/muijf/treelog/blob/main/LICENSE)

**A customizable tree rendering library and CLI for Rust**

*Provides low-level and high-level APIs for rendering hierarchical data structures, similar to `tree`, `npm ls`, or `cargo tree`. Also includes a command-line utility for quick tree visualization.*

[Installation](#installation) • [Quick Start](#quick-start) • [CLI Usage](#cli-usage) • [Documentation](https://docs.rs/treelog) • [Examples](#examples)

</div>

---

<details>
<summary><b>Features</b> - Overview of all available features</summary>

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

</details>

## Installation

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
treelog = "0.0.4"
```

### As a CLI Tool

Install the CLI tool using cargo:

```bash
cargo install treelog --features cli,walkdir,json
```

Or build from source:

```bash
git clone https://github.com/muijf/treelog
cd treelog
cargo build --release --bin treelog --features cli,walkdir,json
```

The CLI requires the `cli` feature and optionally other features depending on the input sources you want to use.

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

<details>
<summary><b>Basic</b> - Simple tree construction and rendering</summary>

```rust
use treelog::{Tree, renderer::write_tree};

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
let mut output = String::new();
write_tree(&mut output, &tree).unwrap();
// root
//  └─ item
```

</details>

<details>
<summary><b>Macro DSL</b> - Declarative syntax for tree construction (requires <code>macro</code> feature)</summary>

The `tree!` macro provides a clean, declarative syntax for constructing trees:

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

</details>

<details>
<summary><b>Builder API</b> - Fluent interface for tree construction (requires <code>builder</code> feature)</summary>

```rust
use treelog::builder::TreeBuilder;

let mut builder = TreeBuilder::new();
builder.node("root").leaf("item1").node("sub").leaf("subitem1").leaf("subitem2").end().leaf("item2");
let tree = builder.build();
println!("{}", tree.render_to_string());
```

</details>

<details>
<summary><b>Low-Level API</b> - Direct tree construction</summary>

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Leaf(vec!["line1".to_string(), "line2".to_string()]),
    Tree::Node("sub".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]),
]);
println!("{}", tree.render_to_string());
```

</details>

<details>
<summary><b>Iterator API</b> - Stream trees line by line (requires <code>iterator</code> feature)</summary>

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

</details>

<details>
<summary><b>Customization</b> - Custom styles and formatters</summary>

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

</details>

<details>
<summary><b>Streaming Large Trees</b> - Efficient streaming for large trees</summary>

```rust,no_run
use treelog::{Tree, TreeIteratorExt};
use std::io::Write;

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
let mut stdout = std::io::stdout();
for line in TreeIteratorExt::lines(&tree) {
    writeln!(stdout, "{}{}", line.prefix, line.content).unwrap();
}
```

</details>

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

</details>

<details>
<summary><b>Tree Path Utilities</b> - Navigate trees by path (requires <code>path</code> feature)</summary>

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
use std::fs;

let tree = Tree::Node("root".to_string(), vec![
    Tree::Leaf(vec!["item".to_string()])
]);

// Create exports directory
fs::create_dir_all("exports").unwrap();

// Export to HTML (with collapsible nodes)
let html = tree.to_html();
fs::write("exports/tree.html", html).unwrap();

// Export to SVG
let svg = tree.to_svg();
fs::write("exports/tree.svg", svg).unwrap();

// Export to Graphviz DOT format
let dot = tree.to_dot();
fs::write("exports/tree.dot", dot).unwrap();
// Render with: dot -Tpng exports/tree.dot -o exports/tree.png
```

</details>

<details>
<summary><b>Library Integrations</b> - Integrations with popular Rust libraries</summary>

### Serialization (serde)

Serialize and deserialize trees to/from JSON and YAML:

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

// JSON serialization (requires `json` feature)
#[cfg(feature = "json")]
{
    let json = tree.to_json().unwrap();
    let deserialized = Tree::from_json(&json).unwrap();
}

// YAML serialization (requires `yaml` feature)
#[cfg(feature = "yaml")]
{
    let yaml = tree.to_yaml().unwrap();
    let deserialized = Tree::from_yaml(&yaml).unwrap();
}
```

### TOML Support

**Serialize/deserialize trees to/from TOML** (requires both `toml` and `serde` features):

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

// Serialize tree to TOML (preserves exact Tree structure)
let toml = tree.to_toml().unwrap();

// Deserialize back
let deserialized = Tree::from_toml(&toml).unwrap();
```

### File System Integration

Build trees from directory structures (requires `walkdir` feature):

```rust
use treelog::Tree;

// Build tree from current directory
let tree = Tree::from_dir(".").unwrap();

// Build tree with maximum depth
let tree = Tree::from_dir_max_depth(".", 2).unwrap();
```

### Graph Integration

Convert between trees and petgraph graphs (requires `petgraph` feature):

```rust
use treelog::Tree;
use petgraph::Graph;

// Tree to graph
let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
let graph: Graph<String, ()> = tree.to_graph();

// Graph to tree
let mut graph = Graph::<String, ()>::new();
let a = graph.add_node("A".to_string());
let b = graph.add_node("B".to_string());
graph.add_edge(a, b, ());
let tree = Tree::from_graph(&graph);
```

### Cargo Metadata Integration

Visualize Cargo dependency trees (requires `cargo-metadata` feature):

```rust
use treelog::Tree;

// Build dependency tree for current project
let tree = Tree::from_cargo_metadata("Cargo.toml").unwrap();

// Build dependency tree for specific package
let tree = Tree::from_cargo_package_deps("treelog", "Cargo.toml").unwrap();
```

### Git Integration

Visualize Git repository structures (requires `git2` feature):

```rust
use treelog::Tree;

// Build tree from Git repository
let tree = Tree::from_git_repo(".").unwrap();

// Build tree from branches
use git2::Repository;
let repo = Repository::open(".").unwrap();
let tree = Tree::from_git_branches(&repo).unwrap();

// Build tree from specific commit
let commit = repo.head().unwrap().peel_to_commit().unwrap();
let tree = Tree::from_git_commit_tree(&repo, &commit).unwrap();
```

### XML/HTML Integration

Visualize XML/HTML DOM trees (requires `roxmltree` feature):

```rust,no_run
use treelog::Tree;

// Build tree from XML string
let xml = r#"<root><child>text</child></root>"#;
let tree = Tree::from_arbitrary_xml(xml).unwrap();

// Build tree from XML file
let tree = Tree::from_arbitrary_xml_file("example.xml").unwrap();
```

### Rust AST Integration

Visualize Rust source code AST (requires `syn` feature):

```rust,no_run
use treelog::Tree;

// Build tree from Rust file
let tree = Tree::from_syn_file("src/lib.rs").unwrap();

// Build tree from syn::File AST
use syn::parse_file;
let ast = parse_file("fn main() {}").unwrap();
let tree = Tree::from_syn_file_ast(&ast);

// Build tree from individual item
use syn::parse_quote;
let item: syn::Item = parse_quote! {
    struct Test { field: i32 }
};
let tree = Tree::from_syn_item(&item);
```

### RON Integration

Serialize and deserialize trees to/from RON (requires `ron` and `serde` features):

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

// Serialize to RON
let ron = tree.to_ron().unwrap();

// Serialize to pretty RON
let ron_pretty = tree.to_ron_pretty().unwrap();

// Deserialize from RON
let deserialized = Tree::from_ron(&ron).unwrap();
```

### Tree-sitter Integration

Visualize tree-sitter parse trees (requires `tree-sitter` feature):

```rust
use treelog::Tree;
use tree_sitter::{Parser, Language};

// Load a language (tree-sitter-rust is available as a dev-dependency for doctests)
let language: Language = tree_sitter_rust::LANGUAGE.into();

let mut parser = Parser::new();
parser.set_language(&language).unwrap();
let source_code = "fn main() {}";
let parse_tree = parser.parse(source_code, None).unwrap();
let tree = Tree::from_tree_sitter(&parse_tree);

// Or parse and convert in one step
let tree = Tree::from_tree_sitter_language(source_code, language).unwrap();
```

### Clap Integration

Visualize command-line argument structures (requires `clap` feature):

```rust
use treelog::Tree;
use clap::{Command, Arg};

let cmd = Command::new("myapp")
    .subcommand(Command::new("subcommand"))
    .arg(Arg::new("input").short('i'));

let tree = Tree::from_clap_command(&cmd);
```

</details>

<details>
<summary><b>Feature Flags</b> - Configure which features to enable</summary>

Most advanced features are behind feature flags to keep the core library lightweight:

```toml
[dependencies]
treelog = { version = "0.0.4", features = ["traversal", "transform", "path", "comparison", "merge", "export"] }
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
- `serde` - Serde serialization support (Serialize/Deserialize traits)
- `json` - JSON serialization/deserialization (requires `serde`)
- `yaml` - YAML serialization/deserialization (requires `serde`)
- `toml` - TOML parsing and tree conversion
- `walkdir` - File system tree building from directories
- `petgraph` - Graph to/from tree conversion
- `cargo-metadata` - Cargo dependency tree visualization
- `git2` - Git repository structure visualization
- `roxmltree` - XML/HTML DOM tree visualization
- `syn` - Rust AST visualization
- `ron` - RON (Rusty Object Notation) serialization
- `tree-sitter` - Tree-sitter parse tree visualization
- `clap` - Command-line argument structure visualization
- `cli` - CLI binary (includes `clap`)

Use `all` feature to enable everything (note: `cli` is separate and must be enabled explicitly for the binary):
```toml
treelog = { version = "0.0.4", features = ["all"] }
```

To build the CLI binary, enable the `cli` feature along with any input source features you need:
```toml
treelog = { version = "0.0.4", features = ["cli", "walkdir", "json"] }
```

</details>

<details>
<summary><b>Performance</b> - Performance characteristics and optimizations</summary>

- **Pre-computed prefixes** - Efficient string buffer capacity estimation
- **Iterator API** - Stream large trees without materializing the entire structure
- **Stack-based traversal** - Memory-efficient tree walking
- **Zero-copy operations** - Most operations work with references where possible

</details>

## CLI Usage

The `treelog` CLI tool provides a convenient way to visualize trees from various sources without writing code. It's built on top of the library and exposes all its features through a command-line interface.

### Basic Usage

```bash
# Visualize a directory structure
treelog from dir . --max-depth 3

# Visualize Cargo dependencies
treelog from cargo

# Visualize Git repository structure
treelog from git .

# Get tree statistics
treelog from dir . --format json | treelog stats
```

### Input Sources

The CLI supports creating trees from various sources:

- **Directory structures**: `treelog from dir <path> [--max-depth <n>]`
- **Cargo metadata**: `treelog from cargo [--package <name>]`
- **Git repositories**: `treelog from git [path] [--branches] [--commit]`
- **XML/HTML files**: `treelog from xml <file>`
- **Rust source files**: `treelog from rust <file>`
- **JSON/YAML/TOML/RON files**: `treelog from json|yaml|toml|ron <file>`

### Rendering Options

```bash
# Use ASCII style
treelog from dir . --style ascii

# Custom style
treelog from dir . --custom-style ">-,<-,| ,   "

# Output to file
treelog from dir . --output tree.txt

# Export to different formats
treelog from dir . --format json > tree.json
treelog from dir . --format html > tree.html
treelog from dir . --format svg > tree.svg
treelog from dir . --format dot > tree.dot
```

### Tree Operations

```bash
# Get statistics
treelog stats < input.json

# Search for nodes
treelog search "pattern" < input.json

# Sort tree
treelog sort --method label < input.json

# Transform tree
treelog transform map-nodes "[{}]" < input.json

# Compare trees
treelog compare tree1.json tree2.json

# Merge trees
treelog merge --strategy append tree1.json tree2.json
```

### Piping and Serialization

The CLI supports piping trees between commands using serialized formats:

```bash
# Create tree and get statistics
treelog from dir . --format json | treelog stats

# Transform and export
treelog from dir . --format json | treelog transform filter "src" | treelog export html > output.html
```

**Note**: When piping between commands, use `--format json` (or `yaml`, `toml`, `ron`) to serialize the tree structure. The default `text` format is for human-readable output only.

### Help

Get help for any command:

```bash
treelog --help
treelog from --help
treelog from dir --help
```

## Examples

The repository includes comprehensive examples demonstrating all features:

**Core Examples:**
- **[`basic`](examples/basic.rs)** - Basic tree construction and rendering
- **[`builder`](examples/builder.rs)** - Using the builder API
- **[`iterator`](examples/iterator.rs)** - Streaming large trees
- **[`macro`](examples/macro.rs)** - Using the macro DSL
- **[`customization`](examples/customization.rs)** - Custom styles and formatters
- **[`complex`](examples/complex.rs)** - Complex tree structures
- **[`file_tree`](examples/file_tree.rs)** - File system tree example

**Advanced Examples:**
- **[`statistics`](examples/statistics.rs)** - Tree statistics and analysis
- **[`traversal`](examples/traversal.rs)** - Tree traversal iterators
- **[`search`](examples/search.rs)** - Tree search and query operations
- **[`transform`](examples/transform.rs)** - Tree transformation operations
- **[`sorting`](examples/sorting.rs)** - Tree sorting operations
- **[`path`](examples/path.rs)** - Tree path utilities
- **[`comparison`](examples/comparison.rs)** - Tree comparison and diff
- **[`merge`](examples/merge.rs)** - Tree merging strategies
- **[`export`](examples/export.rs)** - Export to HTML, SVG, and DOT formats

**Integration Examples:**
- **[`serde`](examples/serde.rs)** - JSON and YAML serialization
- **[`toml`](examples/toml.rs)** - TOML parsing and conversion
- **[`filesystem`](examples/filesystem.rs)** - File system tree building
- **[`petgraph`](examples/petgraph.rs)** - Graph to/from tree conversion
- **[`cargo`](examples/cargo.rs)** - Cargo dependency tree visualization
- **[`git2`](examples/git2.rs)** - Git repository structure visualization
- **[`xml`](examples/xml.rs)** - XML/HTML DOM tree visualization
- **[`syn`](examples/syn.rs)** - Rust AST visualization
- **[`ron`](examples/ron.rs)** - RON serialization
- **[`tree_sitter`](examples/tree_sitter.rs)** - Tree-sitter parse tree visualization
- **[`clap`](examples/clap.rs)** - Command-line argument structure visualization

Run any example with:
```bash
cargo run --example <name> --all-features
```

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
