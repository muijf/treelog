<div align="center">

![TreeLog Banner](images/banner.png)

[![CI](https://github.com/muijf/treelog/workflows/CI/badge.svg)](https://github.com/muijf/treelog/actions)
[![crates.io](https://img.shields.io/crates/v/treelog.svg)](https://crates.io/crates/treelog)
[![docs.rs](https://docs.rs/treelog/badge.svg)](https://docs.rs/treelog)
[![license](https://img.shields.io/crates/l/treelog.svg)](https://github.com/muijf/treelog/blob/main/LICENSE)

**A customizable tree rendering library and CLI for Rust**

*Provides low-level and high-level APIs for rendering hierarchical data structures, similar to `tree`, `npm ls`, or `cargo tree`. Also includes a command-line utility for quick tree visualization.*

[Documentation](https://docs.rs/treelog) • [Examples](examples/)

</div>

---

**Key Features:** Multiple styles • Macro DSL • Builder API • Iterator support • Tree statistics & traversal • Search & filtering • Transformation & sorting • Export to HTML/SVG/DOT • Integrations with JSON/YAML/TOML, filesystem, Git, Cargo, and more

---

## Table of Contents

- [Installation](#installation)
- [Feature Flags](#feature-flags)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Advanced Features](#advanced-features)
- [CLI Usage](#cli-usage)
- [Examples](#examples)
- [Development](#development)
- [License](#license)
- [Contributing](#contributing)

---

## Installation

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
treelog = "0.0.5"
```

### As a CLI Tool

```bash
# Install from crates.io
cargo install treelog --features cli,arbitrary-walkdir,serde-json

# Or build from source
git clone https://github.com/muijf/treelog
cd treelog
cargo build --release --bin treelog --features cli,arbitrary-walkdir,serde-json
```

> **Note**: The CLI requires the `cli` feature. Enable additional features based on the input sources you need. You can use convenience aliases like `walkdir` (for `arbitrary-walkdir`).

## Feature Flags

> Most features are optional to keep the core library lightweight. Enable only what you need.

**Core Features:**
- `builder` - Builder API for constructing trees
- `iterator` - Iterator API for streaming trees
- `macro` - Macro DSL for tree construction
- `formatters` - Custom formatters for nodes and leaves
- `color` - Color output support (requires `colored`)

**Tree Operations:**
- `traversal` - Tree traversal iterators (pre-order, post-order, level-order)
- `transform` - Tree transformation operations (map, filter, prune)
- `path` - Tree path utilities (get by path, flatten)
- `compare` - Tree comparison and diff operations
- `search` - Tree search operations (find nodes/leaves, get paths)
- `sort` - Tree sorting operations (sort by label, depth, custom)
- `stats` - Tree statistics and metrics
- `merge` - Tree merging with different strategies
- `export` - Export to HTML, SVG, and DOT formats

**Exact Serialization (Round-Trip):**
- `serde` - Meta-feature enabling all serde serialization (includes `serde-json`, `serde-yaml`, `serde-toml`, `serde-ron`)
- `serde-json` - JSON serialization/deserialization (Tree ↔ JSON)
- `serde-yaml` - YAML serialization/deserialization (Tree ↔ YAML)
- `serde-toml` - TOML serialization/deserialization (Tree ↔ TOML)
- `serde-ron` - RON serialization/deserialization (Tree ↔ RON)

**Arbitrary Conversion (One-Way):**
- `arbitrary` - Meta-feature enabling all arbitrary conversions
- `arbitrary-json` - Convert any JSON to Tree (requires `serde-json`)
- `arbitrary-yaml` - Convert any YAML to Tree (requires `serde-yaml`)
- `arbitrary-toml` - Convert any TOML to Tree (requires `serde-toml`)
- `arbitrary-xml` - Convert XML/HTML to Tree
- `arbitrary-walkdir` - Build trees from directory structures
- `arbitrary-petgraph` - Convert petgraph graphs to Tree
- `arbitrary-cargo` - Build trees from Cargo metadata
- `arbitrary-git2` - Build trees from Git repositories
- `arbitrary-syn` - Build trees from Rust AST
- `arbitrary-tree-sitter` - Build trees from tree-sitter parse trees
- `arbitrary-clap` - Build trees from clap command structures

**Convenience Aliases:**
- `walkdir` - Alias for `arbitrary-walkdir`
- `petgraph` - Alias for `arbitrary-petgraph`
- `cargo-metadata` - Alias for `arbitrary-cargo`
- `git2` - Alias for `arbitrary-git2`
- `syn` - Alias for `arbitrary-syn`
- `tree-sitter` - Alias for `arbitrary-tree-sitter`
- `clap` - Alias for `arbitrary-clap` (also used by CLI)
- `cli` - CLI binary (includes `clap`)

**Quick examples:**

```toml
# Common feature set
treelog = { version = "0.0.5", features = ["traversal", "transform", "path", "compare", "merge", "export"] }

# Enable everything
treelog = { version = "0.0.5", features = ["all"] }

# CLI with common sources
treelog = { version = "0.0.5", features = ["cli", "walkdir", "serde-json"] }
```

> **Note**: The `cli` feature is separate and must be enabled explicitly for the binary. Use convenience aliases like `walkdir` (for `arbitrary-walkdir`) when available.

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
<summary><b>Basic Tree Construction</b></summary>

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
<summary><b>Macro DSL</b> <code>macro</code> feature</summary>

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
<summary><b>Builder API</b> <code>builder</code> feature</summary>

```rust
use treelog::builder::TreeBuilder;

let mut builder = TreeBuilder::new();
builder.node("root").leaf("item1").node("sub").leaf("subitem1").leaf("subitem2").end().leaf("item2");
let tree = builder.build();
println!("{}", tree.render_to_string());
```

</details>

<details>
<summary><b>Low-Level API</b></summary>

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
<summary><b>Iterator API</b> <code>iterator</code> feature</summary>

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
<summary><b>Custom Styles & Formatters</b></summary>

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

**Formatters** <code>formatters</code> feature:

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
<summary><b>Streaming Large Trees</b></summary>

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
<summary><b>Tree Statistics</b> <code>stats</code> feature</summary>


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
<summary><b>Tree Traversal</b> <code>traversal</code> feature</summary>

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
<summary><b>Tree Search</b> <code>search</code> feature</summary>

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
<summary><b>Tree Transformation</b> <code>transform</code> feature</summary>

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
<summary><b>Tree Sorting</b> <code>sort</code> feature</summary>

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
<summary><b>Tree Path Utilities</b> <code>path</code> feature</summary>

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
<summary><b>Tree Comparison</b> <code>compare</code> feature</summary>

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
        treelog::compare::TreeDiff::OnlyInFirst { path, content } => {
            println!("Only in first: {:?} - {}", path, content);
        }
        treelog::compare::TreeDiff::OnlyInSecond { path, content } => {
            println!("Only in second: {:?} - {}", path, content);
        }
        treelog::compare::TreeDiff::DifferentContent { path, first, second } => {
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
<summary><b>Tree Merging</b> <code>merge</code> feature</summary>

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
<summary><b>Export Formats</b> <code>export</code> feature</summary>

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
<summary><b>Library Integrations</b></summary>

> **Understanding Serialization**: TreeLog provides two approaches for JSON/YAML/TOML/RON:
> - **Arbitrary conversion** (`from_arbitrary_*`) - Converts ANY data structure to Tree (one-way, for visualization)
> - **Exact serialization** (`from_*` / `to_*`) - Preserves exact Tree structure (round-trip, for persistence)

### JSON, YAML, TOML, RON

**Exact serialization** <code>serde-json</code>, <code>serde-yaml</code>, <code>serde-toml</code>, <code>serde-ron</code>:

```rust
use treelog::Tree;

let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);

// Round-trip serialization
    let json = tree.to_json().unwrap();
let restored = Tree::from_json(&json).unwrap();
```

**Arbitrary conversion** <code>arbitrary-json</code>, <code>arbitrary-yaml</code>, <code>arbitrary-toml</code>:

```rust
use treelog::Tree;

// Convert any JSON/YAML/TOML to Tree (one-way)
let json = r#"{"name": "test", "value": 42}"#;
let tree = Tree::from_arbitrary_json(json).unwrap();
```

### File System <code>arbitrary-walkdir</code>

```rust
use treelog::Tree;

// Build tree from current directory
let tree = Tree::from_dir(".").unwrap();

// Build tree with maximum depth
let tree = Tree::from_dir_max_depth(".", 2).unwrap();
```

### Petgraph <code>arbitrary-petgraph</code>

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

### Cargo <code>arbitrary-cargo</code>

```rust
use treelog::Tree;

// Build dependency tree for current project
let tree = Tree::from_cargo_metadata("Cargo.toml").unwrap();

// Build dependency tree for specific package
let tree = Tree::from_cargo_package_deps("treelog", "Cargo.toml").unwrap();
```

### Git <code>arbitrary-git2</code>

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

### XML/HTML <code>arbitrary-xml</code>

```rust,no_run
use treelog::Tree;

// Build tree from XML string
let xml = r#"<root><child>text</child></root>"#;
let tree = Tree::from_arbitrary_xml(xml).unwrap();

// Build tree from XML file
let tree = Tree::from_arbitrary_xml_file("example.xml").unwrap();
```

### Rust AST <code>arbitrary-syn</code>

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

### RON <code>serde-ron</code>

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

### Tree-sitter <code>arbitrary-tree-sitter</code>

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

### Clap <code>arbitrary-clap</code>

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
<summary><b>Performance</b></summary>

- **Pre-computed prefixes** - Efficient string buffer capacity estimation
- **Iterator API** - Stream large trees without materializing the entire structure
- **Stack-based traversal** - Memory-efficient tree walking
- **Zero-copy operations** - Most operations work with references where possible

</details>

## CLI Usage

> The CLI tool provides a convenient way to visualize trees from various sources without writing code. All library features are available through the command-line interface.

### Basic Usage

```bash
# Visualize a directory structure
treelog from dir . --max-depth 3

# Visualize Cargo dependencies
treelog from cargo

# Visualize Git repository structure
treelog from git .

# Render a tree from a file (JSON/YAML/TOML/RON)
treelog render tree.json

# Render a tree from stdin
cat tree.json | treelog render

# Get tree statistics
treelog from dir . --format json | treelog stats
```

### Input Sources

```bash
# Filesystem & Git
treelog from dir <path> [--max-depth <n>]        # arbitrary-walkdir
treelog from git [path] [--branches] [--commit]  # arbitrary-git2

# Code & AST
treelog from rust <file>                         # arbitrary-syn
treelog from tree-sitter <file> [--language]     # arbitrary-tree-sitter
treelog from xml <file>                          # arbitrary-xml

# Package Managers
treelog from cargo [--manifest <path>] [--package <name>]  # arbitrary-cargo

# Data Formats
treelog from json <file>   # serde-json
treelog from yaml <file>   # serde-yaml
treelog from toml <file>   # serde-toml
treelog from ron <file>    # serde-ron
```

### Rendering Options

```bash
# Styles
treelog from dir . --style ascii
treelog from dir . --custom-style ">-,<-,| ,   "

# Output
treelog from dir . --output tree.txt
treelog from dir . --format json > tree.json
treelog from dir . --format html > tree.html
treelog from dir . --format svg > tree.svg
treelog from dir . --format dot > tree.dot
```

### Tree Operations

```bash
# Basic operations
treelog render tree.json          # Render from file
treelog render -                   # Render from stdin
treelog stats tree.json            # Get statistics
treelog search "pattern" tree.json # Search nodes/leaves

# Manipulation
treelog sort --method label tree.json
treelog sort --method depth --reverse tree.json
treelog transform map-nodes "[{}]" tree.json
treelog transform filter "src" tree.json

# Comparison & Merging
treelog compare tree1.json tree2.json
treelog merge --strategy append tree1.json tree2.json

# Export
treelog export html tree.json > output.html
treelog export svg tree.json > output.svg
treelog export dot tree.json > output.dot
```

### Piping and Serialization

```bash
# Create tree and get statistics
treelog from dir . --format json | treelog stats

# Transform and export
treelog from dir . --format json | treelog transform filter "src" | treelog export html > output.html
```

> **Note**: When piping between commands, use `--format json` (or `yaml`, `toml`, `ron`) to serialize the tree structure. The default `text` format is for human-readable output only.

### Help

```bash
treelog --help
treelog from --help
treelog from dir --help
```

## Examples

> Run any example with: `cargo run --example <name> --all-features`

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
- **[`arbitrary`](examples/arbitrary.rs)** - Arbitrary data conversion (JSON/YAML/TOML to Tree)
- **[`serde`](examples/serde.rs)** - Exact JSON and YAML serialization (Tree ↔ JSON/YAML)
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

## Development

**Format code:**
```bash
cargo fmt --all
```
Formats all Rust code according to the official style guide.

**Lint code:**
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
Runs Clippy linter with all targets and features enabled, treating warnings as errors.

**Run tests:**
```bash
cargo test --all-features
```
Runs all tests with all features enabled to ensure comprehensive coverage.

> **Editor setup**: Recommended extensions are available in [`.vscode/extensions.json`](.vscode/extensions.json). See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines and pre-commit hooks.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
