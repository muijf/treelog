//! Internal utilities for tree rendering.

use crate::level::LevelPath;
use crate::style::StyleConfig;

/// Calculates the estimated capacity needed for rendering a tree.
///
/// This is a heuristic that helps pre-allocate string capacity
/// to reduce allocations during rendering.
pub(crate) fn estimate_capacity(tree: &crate::tree::Tree, avg_line_len: usize) -> usize {
    fn count_nodes_and_lines(tree: &crate::tree::Tree) -> (usize, usize) {
        match tree {
            crate::tree::Tree::Node(_, children) => {
                let mut nodes = 1;
                let mut lines = 0;
                for child in children {
                    let (n, l) = count_nodes_and_lines(child);
                    nodes += n;
                    lines += l;
                }
                (nodes, lines)
            }
            crate::tree::Tree::Leaf(leaf_lines) => (0, leaf_lines.len()),
        }
    }

    let (nodes, lines) = count_nodes_and_lines(tree);
    // Estimate: each node/line needs prefix (~10 chars) + content + newline
    (nodes + lines) * (10 + avg_line_len + 1)
}

/// Computes the tree prefix string for a given level path and style.
///
/// This function generates the prefix characters (branches, vertical lines, etc.)
/// that appear before tree node/leaf content when rendering hierarchical structures.
///
/// # Arguments
///
/// * `level` - The path through the tree indicating which ancestors were last children
/// * `style` - The style configuration defining the characters to use
///
/// # Examples
///
/// ```
/// use treelog::{LevelPath, StyleConfig, utils::compute_prefix};
///
/// let level = LevelPath::from_vec(vec![false, true]); // Not last, then last
/// let style = StyleConfig::default();
/// let prefix = compute_prefix(&level, &style);
/// ```
pub fn compute_prefix(level: &LevelPath, style: &StyleConfig) -> String {
    let mut prefix = String::new();
    let maxpos = level.len();
    for (pos, is_last) in level.iter().enumerate() {
        let last_row = pos == maxpos - 1;
        if is_last {
            if !last_row {
                prefix.push_str(style.get_empty());
            } else {
                prefix.push_str(style.get_branch(true));
            }
        } else if !last_row {
            prefix.push_str(style.get_vertical());
        } else {
            prefix.push_str(style.get_branch(false));
        }
    }
    prefix
}

/// Computes the prefix string for continuation lines (e.g., multi-line leaves).
///
/// This is similar to `compute_prefix` but only uses vertical lines and empty spaces,
/// without the branch characters, suitable for lines that continue from a previous line.
///
/// # Arguments
///
/// * `level` - The path through the tree indicating which ancestors were last children
/// * `style` - The style configuration defining the characters to use
///
/// # Examples
///
/// ```
/// use treelog::{LevelPath, StyleConfig, utils::compute_second_line_prefix};
///
/// let level = LevelPath::from_vec(vec![false, true]);
/// let style = StyleConfig::default();
/// let prefix = compute_second_line_prefix(&level, &style);
/// ```
pub fn compute_second_line_prefix(level: &LevelPath, style: &StyleConfig) -> String {
    let mut prefix = String::new();
    for is_last in level.iter() {
        if is_last {
            prefix.push_str(style.get_empty());
        } else {
            prefix.push_str(style.get_vertical());
        }
    }
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Tree;

    #[test]
    fn test_estimate_capacity() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["line1".to_string()]),
                Tree::Leaf(vec!["line2".to_string()]),
            ],
        );
        let capacity = estimate_capacity(&tree, 10);
        assert!(capacity > 0);
    }

    #[test]
    fn test_compute_prefix() {
        let style = StyleConfig::default();

        // Empty path (root)
        let path = LevelPath::new();
        let prefix = compute_prefix(&path, &style);
        assert_eq!(prefix, "");

        // Single level, not last
        let path = LevelPath::from_vec(vec![false]);
        let prefix = compute_prefix(&path, &style);
        assert!(prefix.contains("├─"));

        // Single level, last
        let path = LevelPath::from_vec(vec![true]);
        let prefix = compute_prefix(&path, &style);
        assert!(prefix.contains("└─"));

        // Two levels
        let path = LevelPath::from_vec(vec![false, true]);
        let prefix = compute_prefix(&path, &style);
        assert!(prefix.contains("│"));
        assert!(prefix.contains("└─"));
    }

    #[test]
    fn test_compute_second_line_prefix() {
        let style = StyleConfig::default();

        // Empty path
        let path = LevelPath::new();
        let prefix = compute_second_line_prefix(&path, &style);
        assert_eq!(prefix, "");

        // Single level, not last
        let path = LevelPath::from_vec(vec![false]);
        let prefix = compute_second_line_prefix(&path, &style);
        assert!(prefix.contains("│"));

        // Single level, last
        let path = LevelPath::from_vec(vec![true]);
        let prefix = compute_second_line_prefix(&path, &style);
        assert!(!prefix.contains("│"));
    }
}
