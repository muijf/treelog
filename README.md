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
treelog = "0.1.0-beta.0"
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

> **Note**: The CLI requires the `cli` feature. Enable additional features based on the input sources you need.

## Feature Flags

> Most features are optional to keep the core library lightweight. Enable only what you need.

**Core Features:**
- `iterator` - Iterator API for streaming trees
- `formatters` - Custom formatters for nodes and leaves
- `color` - Color output support (requires `colored`)

**Tree Building:**
- `macro` - Macro DSL for tree construction
- `builder` - Builder API for constructing trees
- `incremental` - Incremental tree construction for dynamic tree building

**Tree Operations:**
- `traversal` - Tree traversal iterators (pre-order, post-order, level-order)
- `transform` - Tree transformation operations (map, filter, prune)
- `path` - Tree path utilities (get by path, flatten)
- `compare` - Tree comparison and diff operations
- `search` - Tree search operations (find nodes/leaves, get paths)
- `sort` - Tree sorting operations (sort by label, depth, custom)
- `stats` - Tree statistics and metrics
- `merge` - Tree merging with different strategies

**Export Formats:**
- `export-html` - Export to HTML format with collapsible nodes
- `export-svg` - Export to SVG tree diagram format
- `export-dot` - Export to Graphviz DOT format

**Exact Serialization (Round-Trip):**
- `serde` - Enables serialization/deserialization support for Tree structures (enables `serde` dependency)
- `serde-json` - JSON serialization/deserialization (Tree ↔ JSON)
- `serde-yaml` - YAML serialization/deserialization (Tree ↔ YAML)
- `serde-toml` - TOML serialization/deserialization (Tree ↔ TOML)
- `serde-ron` - RON serialization/deserialization (Tree ↔ RON)

**Arbitrary Conversion (One-Way):**
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

**CLI:**
- `cli` - CLI binary

**Quick examples:**

```toml
# Common feature set
treelog = { version = "0.1.0-beta.0", features = ["traversal", "transform", "path", "compare", "merge", "export-html", "export-svg", "export-dot"] }

# With serialization support
treelog = { version = "0.1.0-beta.0", features = ["serde", "serde-json", "serde-yaml"] }
```

> **Note**: The `cli` feature is separate and must be enabled explicitly for the binary.

## Quick Start

```rust
use treelog::{Tree, render::write_tree};

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

> For more examples and usage patterns, see the [examples](#examples).

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
