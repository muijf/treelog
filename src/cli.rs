//! Command-line interface for treelog.

use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use std::io::{self, Read};
#[cfg(any(
    feature = "walkdir",
    feature = "cargo-metadata",
    feature = "git2",
    feature = "syn"
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
    Json,
    Yaml,
    Toml,
    Ron,
    Html,
    Svg,
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
    #[cfg(feature = "comparison")]
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
    #[cfg(feature = "walkdir")]
    Dir {
        /// Directory path
        path: PathBuf,
        /// Maximum depth
        #[arg(long)]
        max_depth: Option<usize>,
    },
    /// Build tree from Cargo metadata
    #[cfg(feature = "cargo-metadata")]
    Cargo {
        /// Manifest path (default: Cargo.toml in current directory)
        #[arg(default_value = "Cargo.toml")]
        manifest: PathBuf,
        /// Package name (for specific package dependencies)
        #[arg(long)]
        package: Option<String>,
    },
    /// Build tree from Git repository
    #[cfg(feature = "git2")]
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
    #[cfg(feature = "roxmltree")]
    Xml {
        /// XML/HTML file path (use '-' for stdin)
        file: String,
    },
    /// Build tree from Rust source file (AST)
    #[cfg(feature = "syn")]
    Rust {
        /// Rust source file path
        file: PathBuf,
    },
    /// Build tree from tree-sitter parse
    #[cfg(feature = "tree-sitter")]
    Parse {
        /// Source file path (use '-' for stdin)
        file: String,
        /// Language name (e.g., rust, javascript)
        #[arg(long)]
        language: Option<String>,
    },
    /// Build tree from JSON file
    #[cfg(feature = "json")]
    Json {
        /// JSON file path (use '-' for stdin)
        file: String,
    },
    /// Build tree from YAML file
    #[cfg(feature = "yaml")]
    Yaml {
        /// YAML file path (use '-' for stdin)
        file: String,
    },
    /// Build tree from TOML file
    #[cfg(feature = "toml")]
    Toml {
        /// TOML file path (use '-' for stdin)
        file: String,
    },
    /// Build tree from RON file
    #[cfg(feature = "ron")]
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

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::From { source } => handle_from(source, &cli),
        Commands::Render { input } => handle_render(input, &cli),
        Commands::Stats { input } => handle_stats(input),
        Commands::Search { pattern, input } => handle_search(pattern, input),
        #[cfg(feature = "transform")]
        Commands::Transform { operation, input } => handle_transform(operation, input, &cli),
        Commands::Sort {
            method,
            reverse,
            input,
        } => handle_sort(method, *reverse, input, &cli),
        #[cfg(feature = "comparison")]
        Commands::Compare { first, second } => handle_compare(first, second),
        #[cfg(feature = "merge")]
        Commands::Merge {
            strategy,
            first,
            second,
        } => handle_merge(strategy, first, second, &cli),
        #[cfg(feature = "export")]
        Commands::Export { format, input } => handle_export(format, input),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[allow(unreachable_code, unused_variables)]
fn handle_from(source: &FromSource, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(any(
        feature = "walkdir",
        feature = "cargo-metadata",
        feature = "git2",
        feature = "roxmltree",
        feature = "syn",
        feature = "tree-sitter",
        feature = "json",
        feature = "yaml",
        feature = "toml",
        feature = "ron"
    )))]
    {
        return Err("No input source features enabled. Enable at least one feature (walkdir, cargo-metadata, git2, roxmltree, syn, tree-sitter, json, yaml, toml, or ron).".into());
    }

    #[allow(unreachable_code)]
    let tree = match source {
        #[cfg(feature = "walkdir")]
        FromSource::Dir { path, max_depth } => {
            if let Some(depth) = max_depth {
                treelog::Tree::from_dir_max_depth(path, *depth)?
            } else {
                treelog::Tree::from_dir(path)?
            }
        }
        #[cfg(feature = "cargo-metadata")]
        FromSource::Cargo { manifest, package } => {
            if let Some(pkg) = package {
                treelog::Tree::from_cargo_package_deps(pkg, manifest)?
            } else {
                treelog::Tree::from_cargo_metadata(manifest)?
            }
        }
        #[cfg(feature = "git2")]
        FromSource::Git {
            path,
            branches,
            commit,
        } => {
            use git2::Repository;
            let repo = Repository::open(path)?;
            if *branches {
                treelog::Tree::from_git_branches(&repo)?
            } else if *commit {
                let head = repo.head()?.peel_to_commit()?;
                treelog::Tree::from_git_commit_tree(&repo, &head)?
            } else {
                treelog::Tree::from_git_repo(path)?
            }
        }
        #[cfg(feature = "roxmltree")]
        FromSource::Xml { file } => {
            if file == "-" {
                let mut content = String::new();
                io::stdin().read_to_string(&mut content)?;
                treelog::Tree::from_xml(&content)?
            } else {
                treelog::Tree::from_xml_file(file)?
            }
        }
        #[cfg(feature = "syn")]
        FromSource::Rust { file } => treelog::Tree::from_syn_file(file)?,
        #[cfg(feature = "tree-sitter")]
        FromSource::Parse {
            file: _file,
            language: _language,
        } => {
            return Err("tree-sitter parsing requires language specification. This feature needs implementation.".into());
        }
        #[cfg(feature = "json")]
        FromSource::Json { file } => {
            let content = read_file_or_stdin(file)?;
            treelog::Tree::from_json(&content)?
        }
        #[cfg(feature = "yaml")]
        FromSource::Yaml { file } => {
            let content = read_file_or_stdin(file)?;
            treelog::Tree::from_yaml(&content)?
        }
        #[cfg(feature = "toml")]
        FromSource::Toml { file } => {
            let content = read_file_or_stdin(file)?;
            treelog::Tree::from_toml(&content)?
        }
        #[cfg(feature = "ron")]
        FromSource::Ron { file } => {
            let content = read_file_or_stdin(file)?;
            treelog::Tree::from_ron(&content)?
        }
        #[cfg(not(any(
            feature = "walkdir",
            feature = "cargo-metadata",
            feature = "git2",
            feature = "roxmltree",
            feature = "syn",
            feature = "tree-sitter",
            feature = "json",
            feature = "yaml",
            feature = "toml",
            feature = "ron"
        )))]
        _ => {
            return Err("No input source features enabled. Enable at least one feature (walkdir, cargo-metadata, git2, roxmltree, syn, tree-sitter, json, yaml, toml, or ron).".into());
        }
    };

    output_tree(&tree, cli)
}

fn handle_render(input: &str, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let tree = read_tree(input)?;
    output_tree(&tree, cli)
}

fn handle_stats(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree = read_tree(input)?;
    let stats = tree.stats();
    println!("Tree Statistics:");
    println!("  Depth: {}", stats.depth);
    println!("  Width: {}", stats.width);
    println!("  Node count: {}", stats.node_count);
    println!("  Leaf count: {}", stats.leaf_count);
    println!("  Total lines: {}", stats.total_lines);
    Ok(())
}

fn handle_search(pattern: &str, input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree = read_tree(input)?;
    let matches = tree.find_all_nodes(pattern);
    if matches.is_empty() {
        println!("No nodes found matching '{}'", pattern);
    } else {
        println!("Found {} node(s) matching '{}':", matches.len(), pattern);
        for (i, node) in matches.iter().enumerate() {
            println!("  {}. {}", i + 1, node.label().unwrap_or("(no label)"));
        }
    }
    Ok(())
}

#[allow(unused_variables)]
#[cfg(feature = "transform")]
fn handle_transform(
    operation: &TransformOp,
    input: &str,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree = read_tree(input)?;
    let transformed = match operation {
        TransformOp::MapNodes { expr, .. } => tree.map_nodes(|label| expr.replace("{}", label)),
        TransformOp::MapLeaves { expr, .. } => tree.map_leaves(|line| expr.replace("{}", line)),
        TransformOp::Filter { pattern, .. } => {
            if let Some(filtered) = tree.filter(|t| match t {
                treelog::Tree::Node(label, _) => label.contains(pattern),
                treelog::Tree::Leaf(lines) => lines.iter().any(|l| l.contains(pattern)),
            }) {
                filtered
            } else {
                return Err("Filter resulted in empty tree".into());
            }
        }
        TransformOp::Prune { pattern, .. } => {
            if let Some(pruned) = tree.prune(|t| match t {
                treelog::Tree::Node(label, _) => label.contains(pattern),
                treelog::Tree::Leaf(lines) => lines.iter().any(|l| l.contains(pattern)),
            }) {
                pruned
            } else {
                return Err("Prune resulted in empty tree".into());
            }
        }
    };
    output_tree(&transformed, cli)
}

fn handle_sort(
    method: &SortMethod,
    reverse: bool,
    input: &str,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tree = read_tree(input)?;
    match method {
        SortMethod::Label => {
            tree.sort_by_label();
            if reverse {
                tree.sort_children(&mut |a, b| b.label().cmp(&a.label()));
            }
        }
        SortMethod::Depth => {
            tree.sort_by_depth(reverse);
        }
    }
    output_tree(&tree, cli)
}

#[allow(unused_variables)]
#[cfg(feature = "comparison")]
fn handle_compare(first: &str, second: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree1 = read_tree(first)?;
    let tree2 = read_tree(second)?;

    if tree1.eq_structure(&tree2) {
        println!("Trees have the same structure");
    } else {
        println!("Trees have different structures");
    }

    let diffs = tree1.diff(&tree2);
    if diffs.is_empty() {
        println!("No differences found");
    } else {
        println!("Found {} difference(s):", diffs.len());
        for diff in diffs {
            match diff {
                treelog::comparison::TreeDiff::OnlyInFirst { path, content } => {
                    println!("  Only in first (path: {:?}): {}", path, content);
                }
                treelog::comparison::TreeDiff::OnlyInSecond { path, content } => {
                    println!("  Only in second (path: {:?}): {}", path, content);
                }
                treelog::comparison::TreeDiff::DifferentContent {
                    path,
                    first,
                    second,
                } => {
                    println!("  Different at {:?}: '{}' vs '{}'", path, first, second);
                }
            }
        }
    }
    Ok(())
}

#[allow(unused_variables)]
#[cfg(feature = "merge")]
fn handle_merge(
    strategy: &treelog::merge::MergeStrategy,
    first: &str,
    second: &str,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree1 = read_tree(first)?;
    let tree2 = read_tree(second)?;
    let merged = tree1.merge(tree2, strategy.clone());
    output_tree(&merged, cli)
}

#[allow(unused_variables)]
#[cfg(feature = "export")]
fn handle_export(format: &ExportFormat, input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree = read_tree(input)?;
    let output = match format {
        ExportFormat::Html => tree.to_html(),
        ExportFormat::Svg => tree.to_svg(),
        ExportFormat::Dot => tree.to_dot(),
    };
    println!("{}", output);
    Ok(())
}

#[allow(unused_variables)]
fn read_tree(input: &str) -> Result<treelog::Tree, Box<dyn std::error::Error>> {
    let content = read_file_or_stdin(input)?;

    // Try to deserialize from JSON first
    #[cfg(feature = "json")]
    if let Ok(tree) = treelog::Tree::from_json(&content) {
        return Ok(tree);
    }

    // Try YAML
    #[cfg(feature = "yaml")]
    if let Ok(tree) = treelog::Tree::from_yaml(&content) {
        return Ok(tree);
    }

    // Try TOML
    #[cfg(feature = "toml")]
    if let Ok(tree) = treelog::Tree::from_toml(&content) {
        return Ok(tree);
    }

    // Try RON
    #[cfg(feature = "ron")]
    if let Ok(tree) = treelog::Tree::from_ron(&content) {
        return Ok(tree);
    }

    #[cfg(not(any(feature = "json", feature = "yaml", feature = "toml", feature = "ron")))]
    let _ = content; // Suppress unused variable warning when no features enabled

    Err("Could not parse tree. Ensure the input is valid JSON, YAML, TOML, or RON, or enable the appropriate feature.".into())
}

fn read_file_or_stdin(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if path == "-" {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        Ok(content)
    } else {
        std::fs::read_to_string(path).map_err(|e| e.into())
    }
}

fn output_tree(tree: &treelog::Tree, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let config = build_render_config(cli)?;
    let output = match &cli.format {
        OutputFormat::Text => tree.render_to_string_with_config(&config),
        #[cfg(feature = "export")]
        OutputFormat::Html => tree.to_html(),
        #[cfg(feature = "export")]
        OutputFormat::Svg => tree.to_svg(),
        #[cfg(feature = "export")]
        OutputFormat::Dot => tree.to_dot(),
        #[cfg(feature = "json")]
        OutputFormat::Json => tree
            .to_json_pretty()
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))?,
        #[cfg(feature = "yaml")]
        OutputFormat::Yaml => tree
            .to_yaml()
            .map_err(|e| format!("Failed to serialize to YAML: {}", e))?,
        #[cfg(feature = "toml")]
        OutputFormat::Toml => tree
            .to_toml()
            .map_err(|e| format!("Failed to serialize to TOML: {}", e))?,
        #[cfg(feature = "ron")]
        OutputFormat::Ron => tree
            .to_ron_pretty()
            .map_err(|e| format!("Failed to serialize to RON: {}", e))?,
        #[cfg(not(feature = "export"))]
        OutputFormat::Html | OutputFormat::Svg | OutputFormat::Dot => {
            return Err(
                "Export feature is not enabled. Enable the 'export' feature to use this format."
                    .into(),
            );
        }
        #[cfg(not(feature = "json"))]
        OutputFormat::Json => {
            return Err(
                "JSON feature is not enabled. Enable the 'json' feature to use JSON format.".into(),
            );
        }
        #[cfg(not(feature = "yaml"))]
        OutputFormat::Yaml => {
            return Err(
                "YAML feature is not enabled. Enable the 'yaml' feature to use YAML format.".into(),
            );
        }
        #[cfg(not(feature = "toml"))]
        OutputFormat::Toml => {
            return Err(
                "TOML feature is not enabled. Enable the 'toml' feature to use TOML format.".into(),
            );
        }
        #[cfg(not(feature = "ron"))]
        OutputFormat::Ron => {
            return Err(
                "RON feature is not enabled. Enable the 'ron' feature to use RON format.".into(),
            );
        }
    };

    if let Some(output_path) = &cli.output {
        if output_path == "-" {
            print!("{}", output);
        } else {
            std::fs::write(output_path, output)?;
        }
    } else {
        print!("{}", output);
    }

    Ok(())
}

fn build_render_config(cli: &Cli) -> Result<treelog::RenderConfig, Box<dyn std::error::Error>> {
    use treelog::{RenderConfig, StyleConfig};

    let mut config = RenderConfig::default();

    // Set style
    if let Some(custom) = &cli.custom_style {
        let parts: Vec<&str> = custom.split(',').collect();
        if parts.len() != 4 {
            return Err(
                "Custom style must have 4 comma-separated values: branch,last,vertical,empty"
                    .into(),
            );
        }
        config = config.with_style(StyleConfig::custom(
            parts[0].trim(),
            parts[1].trim(),
            parts[2].trim(),
            parts[3].trim(),
        ));
    } else {
        // cli.style is already a treelog::TreeStyle (Unicode, Ascii, or Box)
        // Custom variant is skipped by ValueEnum, so we can safely use it
        config = config.with_style(cli.style.clone());
    }

    // Set colors
    #[cfg(feature = "color")]
    {
        if cli.color && !cli.no_color {
            config = config.with_colors(true);
        } else if cli.no_color {
            config = config.with_colors(false);
        }
    }

    Ok(config)
}
