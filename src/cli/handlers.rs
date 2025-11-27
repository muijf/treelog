//! Command handler functions.

#[cfg(feature = "export")]
use super::args::ExportFormat;
#[cfg(feature = "transform")]
use super::args::TransformOp;
use super::args::{Cli, FromSource, SortMethod};
use super::utils;
use std::io::{self, Read};

#[allow(unreachable_code, unused_variables)]
pub fn handle_from(source: &FromSource, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(any(
        feature = "arbitrary-walkdir",
        feature = "arbitrary-cargo",
        feature = "arbitrary-git2",
        feature = "arbitrary-xml",
        feature = "arbitrary-syn",
        feature = "arbitrary-tree-sitter",
        feature = "serde-json",
        feature = "serde-yaml",
        feature = "serde-toml",
        feature = "serde-ron"
    )))]
    {
        return Err("No input source features enabled. Enable at least one feature (arbitrary-walkdir, arbitrary-cargo, arbitrary-git2, arbitrary-xml, arbitrary-syn, arbitrary-tree-sitter, serde-json, serde-yaml, serde-toml, or serde-ron).".into());
    }

    #[allow(unreachable_code)]
    let tree = match source {
        #[cfg(feature = "arbitrary-walkdir")]
        FromSource::Dir { path, max_depth } => {
            if let Some(depth) = max_depth {
                treelog::Tree::from_dir_max_depth(path, *depth)?
            } else {
                treelog::Tree::from_dir(path)?
            }
        }
        #[cfg(feature = "arbitrary-cargo")]
        FromSource::Cargo { manifest, package } => {
            if let Some(pkg) = package {
                treelog::Tree::from_cargo_package_deps(pkg, manifest)?
            } else {
                treelog::Tree::from_cargo_metadata(manifest)?
            }
        }
        #[cfg(feature = "arbitrary-git2")]
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
        #[cfg(feature = "arbitrary-xml")]
        FromSource::Xml { file } => {
            if file == "-" {
                let mut content = String::new();
                io::stdin().read_to_string(&mut content)?;
                treelog::Tree::from_arbitrary_xml(&content)?
            } else {
                treelog::Tree::from_arbitrary_xml_file(file)?
            }
        }
        #[cfg(feature = "arbitrary-syn")]
        FromSource::Rust { file } => treelog::Tree::from_syn_file(file)?,
        #[cfg(feature = "arbitrary-tree-sitter")]
        FromSource::TreeSitter {
            file: _file,
            language: _language,
        } => {
            return Err("tree-sitter parsing requires language specification. This feature needs implementation.".into());
        }
        #[cfg(feature = "serde-json")]
        FromSource::Json { file } => {
            let content = utils::read_file_or_stdin(file)?;
            treelog::Tree::from_json(&content)?
        }
        #[cfg(feature = "serde-yaml")]
        FromSource::Yaml { file } => {
            let content = utils::read_file_or_stdin(file)?;
            treelog::Tree::from_yaml(&content)?
        }
        #[cfg(feature = "serde-toml")]
        FromSource::Toml { file } => {
            let content = utils::read_file_or_stdin(file)?;
            treelog::Tree::from_toml(&content)?
        }
        #[cfg(feature = "serde-ron")]
        FromSource::Ron { file } => {
            let content = utils::read_file_or_stdin(file)?;
            treelog::Tree::from_ron(&content)?
        }
        #[cfg(not(any(
            feature = "arbitrary-walkdir",
            feature = "arbitrary-cargo",
            feature = "arbitrary-git2",
            feature = "arbitrary-xml",
            feature = "arbitrary-syn",
            feature = "arbitrary-tree-sitter",
            feature = "serde-json",
            feature = "serde-yaml",
            feature = "serde-toml",
            feature = "serde-ron"
        )))]
        _ => {
            return Err("No input source features enabled. Enable at least one feature (walkdir, cargo-metadata, git2, arbitrary-xml, syn, tree-sitter, serde-json, serde-yaml, serde-toml, or serde-ron).".into());
        }
    };

    utils::output_tree(&tree, cli)
}

pub fn handle_render(input: &str, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let tree = utils::read_tree(input)?;
    utils::output_tree(&tree, cli)
}

pub fn handle_stats(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree = utils::read_tree(input)?;
    let stats = tree.stats();
    println!("Tree Statistics:");
    println!("  Depth: {}", stats.depth);
    println!("  Width: {}", stats.width);
    println!("  Node count: {}", stats.node_count);
    println!("  Leaf count: {}", stats.leaf_count);
    println!("  Total lines: {}", stats.total_lines);
    Ok(())
}

pub fn handle_search(pattern: &str, input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree = utils::read_tree(input)?;
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
pub fn handle_transform(
    operation: &TransformOp,
    input: &str,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree = utils::read_tree(input)?;
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
    utils::output_tree(&transformed, cli)
}

pub fn handle_sort(
    method: &SortMethod,
    reverse: bool,
    input: &str,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tree = utils::read_tree(input)?;
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
    utils::output_tree(&tree, cli)
}

#[allow(unused_variables)]
#[cfg(feature = "compare")]
pub fn handle_compare(first: &str, second: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree1 = utils::read_tree(first)?;
    let tree2 = utils::read_tree(second)?;

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
                treelog::compare::TreeDiff::OnlyInFirst { path, content } => {
                    println!("  Only in first (path: {:?}): {}", path, content);
                }
                treelog::compare::TreeDiff::OnlyInSecond { path, content } => {
                    println!("  Only in second (path: {:?}): {}", path, content);
                }
                treelog::compare::TreeDiff::DifferentContent {
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
pub fn handle_merge(
    strategy: &treelog::merge::MergeStrategy,
    first: &str,
    second: &str,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    let tree1 = utils::read_tree(first)?;
    let tree2 = utils::read_tree(second)?;
    let merged = tree1.merge(tree2, strategy.clone());
    utils::output_tree(&merged, cli)
}

#[allow(unused_variables)]
#[cfg(feature = "export")]
pub fn handle_export(format: &ExportFormat, input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tree = utils::read_tree(input)?;
    let output = match format {
        ExportFormat::Html => tree.to_html(),
        ExportFormat::Svg => tree.to_svg(),
        ExportFormat::Dot => tree.to_dot(),
    };
    println!("{}", output);
    Ok(())
}
