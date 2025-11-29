//! Tree statistics and metrics.

use crate::tree::Tree;

/// Statistics about a tree structure.
///
/// This struct provides various metrics about a tree, including its depth,
/// width, and counts of nodes and leaves.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeStats {
    /// Maximum depth of the tree
    pub depth: usize,
    /// Maximum width at any level
    pub width: usize,
    /// Total number of nodes
    pub node_count: usize,
    /// Total number of leaves
    pub leaf_count: usize,
    /// Total number of lines across all leaves
    pub total_lines: usize,
}

impl Tree {
    /// Returns the maximum depth of the tree.
    ///
    /// The depth is the number of levels from the root to the deepest node.
    /// A single node or leaf has depth 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// assert_eq!(tree.depth(), 1);
    /// ```
    pub fn depth(&self) -> usize {
        match self {
            Tree::Node(_, children) => {
                if children.is_empty() {
                    0
                } else {
                    1 + children
                        .iter()
                        .map(|child| child.depth())
                        .max()
                        .unwrap_or(0)
                }
            }
            Tree::Leaf(_) => 0,
        }
    }

    /// Returns the maximum width at any level of the tree.
    ///
    /// The width is the maximum number of children at any single level.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["a".to_string()]),
    ///     Tree::Leaf(vec!["b".to_string()]),
    ///     Tree::Leaf(vec!["c".to_string()]),
    /// ]);
    /// assert_eq!(tree.width(), 3);
    /// ```
    pub fn width(&self) -> usize {
        let mut level_widths = Vec::new();
        self.collect_widths_at_level(0, &mut level_widths);
        // Width is the maximum number of children at any level
        level_widths.into_iter().max().unwrap_or(0)
    }

    fn collect_widths_at_level(&self, level: usize, widths: &mut Vec<usize>) {
        match self {
            Tree::Node(_, children) => {
                // Count children at this level
                if level >= widths.len() {
                    widths.resize(level + 1, 0);
                }
                widths[level] = widths[level].max(children.len());
                for child in children {
                    child.collect_widths_at_level(level + 1, widths);
                }
            }
            Tree::Leaf(_) => {
                // Leaves don't contribute to width at their level
            }
        }
    }

    /// Returns the total number of nodes in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("child".to_string(), vec![
    ///         Tree::Leaf(vec!["leaf".to_string()])
    ///     ])
    /// ]);
    /// assert_eq!(tree.node_count(), 2);
    /// ```
    pub fn node_count(&self) -> usize {
        match self {
            Tree::Node(_, children) => {
                1 + children
                    .iter()
                    .map(|child| child.node_count())
                    .sum::<usize>()
            }
            Tree::Leaf(_) => 0,
        }
    }

    /// Returns the total number of leaves in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["a".to_string()]),
    ///     Tree::Leaf(vec!["b".to_string()]),
    /// ]);
    /// assert_eq!(tree.leaf_count(), 2);
    /// ```
    pub fn leaf_count(&self) -> usize {
        match self {
            Tree::Node(_, children) => children.iter().map(|child| child.leaf_count()).sum(),
            Tree::Leaf(_) => 1,
        }
    }

    /// Returns the total number of lines across all leaves in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["line1".to_string(), "line2".to_string()]),
    ///     Tree::Leaf(vec!["line3".to_string()]),
    /// ]);
    /// assert_eq!(tree.total_lines(), 3);
    /// ```
    pub fn total_lines(&self) -> usize {
        match self {
            Tree::Node(_, children) => children.iter().map(|child| child.total_lines()).sum(),
            Tree::Leaf(lines) => lines.len(),
        }
    }

    /// Returns statistics about the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// let stats = tree.stats();
    /// assert_eq!(stats.depth, 1);
    /// assert_eq!(stats.node_count, 1);
    /// assert_eq!(stats.leaf_count, 1);
    /// ```
    pub fn stats(&self) -> TreeStats {
        TreeStats {
            depth: self.depth(),
            width: self.width(),
            node_count: self.node_count(),
            leaf_count: self.leaf_count(),
            total_lines: self.total_lines(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        assert_eq!(tree.depth(), 1);

        let deep_tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![Tree::Node(
                    "grandchild".to_string(),
                    vec![Tree::Leaf(vec!["leaf".to_string()])],
                )],
            )],
        );
        assert_eq!(deep_tree.depth(), 3);

        let single_node = Tree::new_node("root");
        assert_eq!(single_node.depth(), 0);

        let single_leaf = Tree::new_leaf("leaf");
        assert_eq!(single_leaf.depth(), 0);
    }

    #[test]
    fn test_width() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["a".to_string()]),
                Tree::Leaf(vec!["b".to_string()]),
                Tree::Leaf(vec!["c".to_string()]),
            ],
        );
        assert_eq!(tree.width(), 3);

        let tree2 = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node(
                    "child1".to_string(),
                    vec![
                        Tree::Leaf(vec!["gc1".to_string()]),
                        Tree::Leaf(vec!["gc2".to_string()]),
                    ],
                ),
                Tree::Node(
                    "child2".to_string(),
                    vec![Tree::Leaf(vec!["gc3".to_string()])],
                ),
            ],
        );
        assert_eq!(tree2.width(), 2);

        let single_leaf = Tree::new_leaf("leaf");
        assert_eq!(single_leaf.width(), 0);
    }

    #[test]
    fn test_node_count() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        assert_eq!(tree.node_count(), 1);

        let tree2 = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![Tree::Leaf(vec!["leaf".to_string()])],
            )],
        );
        assert_eq!(tree2.node_count(), 2);

        let leaf = Tree::new_leaf("leaf");
        assert_eq!(leaf.node_count(), 0);
    }

    #[test]
    fn test_leaf_count() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["a".to_string()]),
                Tree::Leaf(vec!["b".to_string()]),
            ],
        );
        assert_eq!(tree.leaf_count(), 2);

        let tree2 = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![
                    Tree::Leaf(vec!["a".to_string()]),
                    Tree::Leaf(vec!["b".to_string()]),
                ],
            )],
        );
        assert_eq!(tree2.leaf_count(), 2);

        let leaf = Tree::new_leaf("leaf");
        assert_eq!(leaf.leaf_count(), 1);
    }

    #[test]
    fn test_total_lines() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["line1".to_string(), "line2".to_string()]),
                Tree::Leaf(vec!["line3".to_string()]),
            ],
        );
        assert_eq!(tree.total_lines(), 3);

        let leaf = Tree::new_leaf("single");
        assert_eq!(leaf.total_lines(), 1);
    }

    #[test]
    fn test_stats() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node(
                    "child".to_string(),
                    vec![Tree::Leaf(vec!["leaf".to_string()])],
                ),
                Tree::Leaf(vec!["leaf2".to_string()]),
            ],
        );
        let stats = tree.stats();
        assert_eq!(stats.depth, 2);
        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.leaf_count, 2);
        assert_eq!(stats.total_lines, 2);
        assert_eq!(stats.width, 2);
    }
}
