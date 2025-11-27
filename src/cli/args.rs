//! CLI argument definitions.

use clap::{ArgAction, Parser, Subcommand, ValueEnum};
#[cfg(any(
    feature = "arbitrary-walkdir",
    feature = "arbitrary-cargo",
    feature = "arbitrary-git2",
    feature = "arbitrary-syn"
))]
use std::path::PathBuf;

#[derive(Clone, Debug, ValueEnum)]
pub enum SortMethod {
    Label,
    Depth,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Text,
    #[cfg(feature = "serde-json")]
    Json,
    #[cfg(feature = "serde-yaml")]
    Yaml,
    #[cfg(feature = "serde-toml")]
    Toml,
    #[cfg(feature = "serde-ron")]
    Ron,
    #[cfg(feature = "export")]
    Html,
    #[cfg(feature = "export")]
    Svg,
    #[cfg(feature = "export")]
    Dot,
}

#[derive(Parser)]
#[command(name = "treelog")]
#[command(about = "A customizable tree rendering library for Rust")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Tree style
    #[arg(long, global = true, value_enum, default_value = "unicode")]
    pub style: treelog::TreeStyle,

    /// Custom style characters (format: branch,last,vertical,empty)
    #[arg(long, global = true)]
    pub custom_style: Option<String>,

    /// Enable color output
    #[arg(long, global = true, action = ArgAction::SetTrue)]
    pub color: bool,

    /// Disable color output
    #[arg(long, global = true, action = ArgAction::SetTrue)]
    pub no_color: bool,

    /// Output file (use '-' for stdout)
    #[arg(short, long, global = true)]
    pub output: Option<String>,

    /// Output format
    #[arg(long, global = true, value_enum, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create tree from various sources
    From {
        #[command(subcommand)]
        source: FromSource,
    },
    /// Render a tree (from stdin or file)
    Render {
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
    /// Display tree statistics
    Stats {
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
    /// Search for nodes/leaves matching pattern
    Search {
        /// Search pattern
        pattern: String,
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
    /// Transform tree operations
    #[cfg(feature = "transform")]
    Transform {
        #[command(subcommand)]
        operation: TransformOp,
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
    /// Sort tree children
    Sort {
        /// Sort method
        #[arg(value_enum, default_value = "label")]
        method: SortMethod,
        /// Reverse sort order
        #[arg(short, long)]
        reverse: bool,
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
    /// Compare two trees
    #[cfg(feature = "compare")]
    Compare {
        /// First tree file
        first: String,
        /// Second tree file
        second: String,
    },
    /// Merge two trees
    #[cfg(feature = "merge")]
    Merge {
        /// First tree file
        first: String,
        /// Second tree file
        second: String,
        /// Merge strategy
        #[arg(long, value_enum, default_value = "append")]
        strategy: treelog::merge::MergeStrategy,
    },
    /// Export tree to various formats
    #[cfg(feature = "export")]
    Export {
        #[command(subcommand)]
        format: ExportFormat,
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
}

#[derive(Subcommand)]
pub enum FromSource {
    /// Build tree from directory structure
    #[cfg(feature = "arbitrary-walkdir")]
    Dir {
        /// Directory path
        path: PathBuf,
        /// Maximum depth
        #[arg(long)]
        max_depth: Option<usize>,
    },
    /// Build tree from Cargo metadata
    #[cfg(feature = "arbitrary-cargo")]
    Cargo {
        /// Manifest path (default: Cargo.toml in current directory)
        #[arg(default_value = "Cargo.toml")]
        manifest: PathBuf,
        /// Package name (for specific package dependencies)
        #[arg(long)]
        package: Option<String>,
    },
    /// Build tree from Git repository
    #[cfg(feature = "arbitrary-git2")]
    Git {
        /// Repository path (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Show branches only
        #[arg(long)]
        branches: bool,
        /// Show commit tree only
        #[arg(long)]
        commit: bool,
    },
    /// Build tree from XML/HTML file
    #[cfg(feature = "arbitrary-xml")]
    Xml {
        /// XML/HTML file path (use '-' for stdin)
        file: String,
    },
    /// Build tree from Rust source file (AST)
    #[cfg(feature = "arbitrary-syn")]
    Rust {
        /// Rust source file path
        file: PathBuf,
    },
    /// Build tree from tree-sitter parse
    #[cfg(feature = "arbitrary-tree-sitter")]
    TreeSitter {
        /// Source file path (use '-' for stdin)
        file: String,
        /// Language name (e.g., rust, javascript)
        #[arg(long)]
        language: Option<String>,
    },
    /// Build tree from JSON file
    #[cfg(feature = "serde-json")]
    Json {
        /// JSON file path (use '-' for stdin)
        file: String,
    },
    /// Build tree from YAML file
    #[cfg(feature = "serde-yaml")]
    Yaml {
        /// YAML file path (use '-' for stdin)
        file: String,
    },
    /// Build tree from TOML file
    #[cfg(feature = "serde-toml")]
    Toml {
        /// TOML file path (use '-' for stdin)
        file: String,
    },
    /// Build tree from RON file
    #[cfg(feature = "serde-ron")]
    Ron {
        /// RON file path (use '-' for stdin)
        file: String,
    },
}

#[derive(Subcommand)]
#[cfg(feature = "transform")]
pub enum TransformOp {
    /// Map node labels
    MapNodes {
        /// Transformation expression (e.g., "[{}]")
        expr: String,
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
    /// Map leaf lines
    MapLeaves {
        /// Transformation expression (e.g., "- {}")
        expr: String,
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
    /// Filter tree
    Filter {
        /// Filter pattern
        pattern: String,
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
    /// Prune tree
    Prune {
        /// Prune pattern
        pattern: String,
        /// Input file (use '-' for stdin)
        #[arg(default_value = "-")]
        input: String,
    },
}

#[derive(Subcommand)]
#[cfg(feature = "export")]
pub enum ExportFormat {
    /// Export to HTML
    Html,
    /// Export to SVG
    Svg,
    /// Export to Graphviz DOT
    Dot,
}
