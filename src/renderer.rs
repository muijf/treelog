//! Core rendering logic for trees.

use std::fmt;
use std::fmt::Write;

use crate::config::RenderConfig;
use crate::level::LevelPath;
use crate::tree::Tree;
use crate::utils::estimate_capacity;

/// Renders a tree to a writer using the default configuration.
///
/// # Examples
///
/// ```
/// use treelog::Tree;
/// use treelog::renderer::write_tree;
///
/// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
/// let mut output = String::new();
/// write_tree(&mut output, &tree).unwrap();
/// ```
pub fn write_tree(f: &mut dyn Write, tree: &Tree) -> fmt::Result {
    write_tree_with_config(f, tree, &RenderConfig::default())
}

/// Renders a tree to a writer using a custom configuration.
///
/// # Examples
///
/// ```
/// use treelog::{Tree, TreeStyle, RenderConfig};
/// use treelog::renderer::write_tree_with_config;
///
/// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
/// let config = RenderConfig::default().with_style(TreeStyle::Ascii);
/// let mut output = String::new();
/// write_tree_with_config(&mut output, &tree, &config).unwrap();
/// ```
pub fn write_tree_with_config(
    f: &mut dyn Write,
    tree: &Tree,
    config: &RenderConfig,
) -> fmt::Result {
    write_tree_element(f, tree, &LevelPath::new(), config)
}

fn write_tree_element(
    f: &mut dyn Write,
    tree: &Tree,
    level: &LevelPath,
    config: &RenderConfig,
) -> fmt::Result {
    let style = &config.style;
    let maxpos = level.len();
    let mut second_line = String::new();

    // Build the prefix for the current line
    for (pos, is_last) in level.iter().enumerate() {
        let last_row = pos == maxpos - 1;
        if is_last {
            // This branch is the last child at this level
            if !last_row {
                write!(f, "{}", style.get_empty())?;
            } else {
                write!(f, "{}", style.get_branch(true))?;
            }
            second_line.push_str(style.get_empty());
        } else {
            // This branch is not the last child
            if !last_row {
                write!(f, "{}", style.get_vertical())?;
            } else {
                write!(f, "{}", style.get_branch(false))?;
            }
            second_line.push_str(style.get_vertical());
        }
    }

    match tree {
        Tree::Node(label, children) => {
            let formatted_label = config.format_node(label);
            let final_label = if config.colors {
                #[cfg(feature = "color")]
                {
                    use colored::Colorize;
                    formatted_label.blue().to_string()
                }
                #[cfg(not(feature = "color"))]
                {
                    formatted_label
                }
            } else {
                formatted_label
            };
            write!(f, "{}{}", final_label, config.line_ending)?;

            let mut remaining = children.len();
            for child in children {
                let is_last = remaining == 1;
                let lnext = level.with_child(is_last);
                remaining -= 1;
                write_tree_element(f, child, &lnext, config)?;
            }
        }
        Tree::Leaf(lines) => {
            for (i, line) in lines.iter().enumerate() {
                let formatted_line = config.format_leaf(line);
                let final_line = if config.colors {
                    #[cfg(feature = "color")]
                    {
                        use colored::Colorize;
                        formatted_line.green().to_string()
                    }
                    #[cfg(not(feature = "color"))]
                    {
                        formatted_line
                    }
                } else {
                    formatted_line
                };
                if i == 0 {
                    writeln!(f, "{}{}", final_line, config.line_ending.trim_end())?;
                } else {
                    writeln!(
                        f,
                        "{} {}{}",
                        second_line,
                        final_line,
                        config.line_ending.trim_end()
                    )?;
                }
            }
        }
    }

    Ok(())
}

/// Renders a tree to a String using the default configuration.
///
/// # Examples
///
/// ```
/// use treelog::Tree;
/// use treelog::renderer::render_to_string;
///
/// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
/// let output = render_to_string(&tree);
/// ```
pub fn render_to_string(tree: &Tree) -> String {
    render_to_string_with_config(tree, &RenderConfig::default())
}

/// Renders a tree to a String using a custom configuration.
///
/// # Examples
///
/// ```
/// use treelog::{Tree, TreeStyle, RenderConfig};
/// use treelog::renderer::render_to_string_with_config;
///
/// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
/// let config = RenderConfig::default().with_style(TreeStyle::Ascii);
/// let output = render_to_string_with_config(&tree, &config);
/// ```
pub fn render_to_string_with_config(tree: &Tree, config: &RenderConfig) -> String {
    let capacity = estimate_capacity(tree, 20);
    let mut output = String::with_capacity(capacity);
    write_tree_with_config(&mut output, tree, config).unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::TreeStyle;

    #[test]
    fn test_write_tree() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let mut output = String::new();
        write_tree(&mut output, &tree).unwrap();
        assert!(output.contains("root"));
        assert!(output.contains("item"));
    }

    #[test]
    fn test_write_tree_with_config() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let config = RenderConfig::default().with_style(TreeStyle::Ascii);
        let mut output = String::new();
        write_tree_with_config(&mut output, &tree, &config).unwrap();
        assert!(output.contains("root"));
        assert!(output.contains("item"));
    }

    #[test]
    fn test_render_to_string() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let output = render_to_string(&tree);
        assert!(output.contains("root"));
        assert!(output.contains("item"));
    }

    #[test]
    fn test_complex_tree() {
        let l1 = Tree::Leaf(vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ]);
        let l2 = Tree::Leaf(vec!["only one line".to_string()]);
        let n1 = Tree::Node("node 1".to_string(), vec![l1.clone(), l2.clone()]);
        let n2 = Tree::Node("node 2".to_string(), vec![l2.clone(), l1.clone()]);
        let n3 = Tree::Node(
            "node 3".to_string(),
            vec![n1.clone(), l1.clone(), l2.clone()],
        );
        let n4 = Tree::Node("node 4".to_string(), vec![n1, n2, n3]);

        let output = render_to_string(&n4);
        assert!(output.contains("node 4"));
        assert!(output.contains("node 1"));
        assert!(output.contains("node 2"));
        assert!(output.contains("node 3"));
    }
}
