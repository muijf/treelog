//! Core tree data structure for representing hierarchical data.

use std::fmt;

#[cfg(any(feature = "traversal", doc))]
use crate::traversal::{Leaves, LevelOrder, Nodes, PostOrder, PreOrder};

/// A tree structure that can represent hierarchical data with nodes and leaves.
///
/// Nodes contain a label and children, while leaves contain one or more lines of text.
///
/// # Examples
///
/// ```
/// use treelog::Tree;
///
/// let tree = Tree::Node(
///     "root".to_string(),
///     vec![
///         Tree::Leaf(vec!["item1".to_string(), "item2".to_string()]),
///         Tree::Node("sub".to_string(), vec![Tree::Leaf(vec!["subitem".to_string()])]),
///     ],
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Tree {
    /// A node with a label and child trees.
    Node(String, Vec<Tree>),
    /// A leaf containing one or more lines of text.
    Leaf(Vec<String>),
}

impl Tree {
    /// Creates a new node with the given label and no children.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let node = Tree::new_node("root");
    /// ```
    #[inline]
    pub fn new_node(label: impl Into<String>) -> Self {
        Tree::Node(label.into(), Vec::new())
    }

    /// Creates a new leaf with a single line of text.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let leaf = Tree::new_leaf("single line");
    /// ```
    #[inline]
    pub fn new_leaf(line: impl Into<String>) -> Self {
        Tree::Leaf(vec![line.into()])
    }

    /// Creates a new leaf with multiple lines of text.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let leaf = Tree::new_leaf_lines(vec!["line1", "line2", "line3"]);
    /// ```
    #[inline]
    pub fn new_leaf_lines(lines: Vec<impl Into<String>>) -> Self {
        Tree::Leaf(lines.into_iter().map(Into::into).collect())
    }

    /// Adds a child to this tree. Returns `Some(self)` if this is a node, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let mut node = Tree::new_node("root");
    /// node.add_child(Tree::new_leaf("child"));
    /// ```
    pub fn add_child(&mut self, child: Tree) -> Option<&mut Self> {
        match self {
            Tree::Node(_, children) => {
                children.push(child);
                Some(self)
            }
            Tree::Leaf(_) => None,
        }
    }

    /// Returns the number of direct children if this is a node, or `None` if it's a leaf.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let mut node = Tree::new_node("root");
    /// node.add_child(Tree::new_leaf("child1"));
    /// node.add_child(Tree::new_leaf("child2"));
    /// assert_eq!(node.child_count(), Some(2));
    /// ```
    #[inline]
    pub fn child_count(&self) -> Option<usize> {
        match self {
            Tree::Node(_, children) => Some(children.len()),
            Tree::Leaf(_) => None,
        }
    }

    /// Returns `true` if this is a node, `false` if it's a leaf.
    #[inline]
    pub fn is_node(&self) -> bool {
        matches!(self, Tree::Node(_, _))
    }

    /// Returns `true` if this is a leaf, `false` if it's a node.
    #[inline]
    pub fn is_leaf(&self) -> bool {
        matches!(self, Tree::Leaf(_))
    }

    /// Returns a reference to the label if this is a node, or `None` if it's a leaf.
    #[inline]
    pub fn label(&self) -> Option<&str> {
        match self {
            Tree::Node(label, _) => Some(label),
            Tree::Leaf(_) => None,
        }
    }

    /// Returns a reference to the lines if this is a leaf, or `None` if it's a node.
    #[inline]
    pub fn lines(&self) -> Option<&[String]> {
        match self {
            Tree::Leaf(lines) => Some(lines),
            Tree::Node(_, _) => None,
        }
    }

    /// Returns a mutable reference to the children if this is a node, or `None` if it's a leaf.
    #[inline]
    pub fn children_mut(&mut self) -> Option<&mut Vec<Tree>> {
        match self {
            Tree::Node(_, children) => Some(children),
            Tree::Leaf(_) => None,
        }
    }

    /// Returns a reference to the children if this is a node, or `None` if it's a leaf.
    #[inline]
    pub fn children(&self) -> Option<&[Tree]> {
        match self {
            Tree::Node(_, children) => Some(children),
            Tree::Leaf(_) => None,
        }
    }

    /// Returns the maximum depth of the tree.
    ///
    /// The depth of a tree is the length of the longest path from the root to a leaf.
    /// A tree with only a root node has depth 0.
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
    /// assert_eq!(tree.depth(), 2);
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

    /// Returns an iterator that traverses the tree in pre-order (root, then children).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for node in tree.pre_order() {
    ///     println!("{:?}", node);
    /// }
    /// ```
    #[cfg(any(feature = "traversal", doc))]
    pub fn pre_order(&self) -> PreOrder<'_> {
        PreOrder::new(self)
    }

    /// Returns an iterator that traverses the tree in post-order (children, then root).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for node in tree.post_order() {
    ///     println!("{:?}", node);
    /// }
    /// ```
    #[cfg(any(feature = "traversal", doc))]
    pub fn post_order(&self) -> PostOrder<'_> {
        PostOrder::new(self)
    }

    /// Returns an iterator that traverses the tree in level-order (breadth-first).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for node in tree.level_order() {
    ///     println!("{:?}", node);
    /// }
    /// ```
    #[cfg(any(feature = "traversal", doc))]
    pub fn level_order(&self) -> LevelOrder<'_> {
        LevelOrder::new(self)
    }

    /// Returns an iterator over all nodes (excluding leaves).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for node in tree.nodes() {
    ///     println!("Node: {}", node.label().unwrap());
    /// }
    /// ```
    #[cfg(any(feature = "traversal", doc))]
    pub fn nodes(&self) -> Nodes<'_> {
        Nodes::new(self)
    }

    /// Returns an iterator over all leaves (excluding nodes).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for leaf in tree.leaves() {
    ///     println!("Leaf: {:?}", leaf.lines());
    /// }
    /// ```
    #[cfg(any(feature = "traversal", doc))]
    pub fn leaves(&self) -> Leaves<'_> {
        Leaves::new(self)
    }

    /// Finds the first node with the given label.
    ///
    /// Returns `Some(&Tree)` if found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("child".to_string(), vec![Tree::Leaf(vec!["item".to_string()])])
    /// ]);
    /// let found = tree.find_node("child");
    /// assert!(found.is_some());
    /// ```
    pub fn find_node(&self, label: &str) -> Option<&Tree> {
        if let Tree::Node(node_label, _) = self
            && node_label == label
        {
            return Some(self);
        }

        if let Tree::Node(_, children) = self {
            for child in children {
                if let Some(found) = child.find_node(label) {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Finds all nodes with the given label.
    ///
    /// Returns a vector of references to matching nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("child".to_string(), vec![]),
    ///     Tree::Node("child".to_string(), vec![]),
    /// ]);
    /// let found = tree.find_all_nodes("child");
    /// assert_eq!(found.len(), 2);
    /// ```
    pub fn find_all_nodes(&self, label: &str) -> Vec<&Tree> {
        let mut results = Vec::new();
        self.collect_nodes(label, &mut results);
        results
    }

    fn collect_nodes<'a>(&'a self, label: &str, results: &mut Vec<&'a Tree>) {
        if let Tree::Node(node_label, _) = self
            && node_label == label
        {
            results.push(self);
        }

        if let Tree::Node(_, children) = self {
            for child in children {
                child.collect_nodes(label, results);
            }
        }
    }

    /// Finds the first leaf containing the given content.
    ///
    /// Returns `Some(&Tree)` if found, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// let found = tree.find_leaf("item");
    /// assert!(found.is_some());
    /// ```
    pub fn find_leaf(&self, content: &str) -> Option<&Tree> {
        match self {
            Tree::Leaf(lines) => {
                if lines.iter().any(|line| line.contains(content)) {
                    return Some(self);
                }
            }
            Tree::Node(_, children) => {
                for child in children {
                    if let Some(found) = child.find_leaf(content) {
                        return Some(found);
                    }
                }
            }
        }
        None
    }

    /// Checks if the tree contains a node or leaf with the given label/content.
    ///
    /// Returns `true` if found, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// assert!(tree.contains("root"));
    /// assert!(tree.contains("item"));
    /// ```
    pub fn contains(&self, label: &str) -> bool {
        match self {
            Tree::Node(node_label, _) => {
                if node_label == label {
                    return true;
                }
            }
            Tree::Leaf(lines) => {
                if lines.iter().any(|line| line.contains(label)) {
                    return true;
                }
            }
        }

        if let Tree::Node(_, children) = self {
            for child in children {
                if child.contains(label) {
                    return true;
                }
            }
        }

        false
    }

    /// Gets the path (indices) to the first node with the given label.
    ///
    /// Returns `Some(Vec<usize>)` if found, where each usize is the index of a child.
    /// Returns `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("child".to_string(), vec![Tree::Leaf(vec!["item".to_string()])])
    /// ]);
    /// let path = tree.path_to("child");
    /// assert_eq!(path, Some(vec![0]));
    /// ```
    pub fn path_to(&self, label: &str) -> Option<Vec<usize>> {
        let mut path = Vec::new();
        if self.find_path(label, &mut path) {
            Some(path)
        } else {
            None
        }
    }

    fn find_path(&self, label: &str, path: &mut Vec<usize>) -> bool {
        match self {
            Tree::Node(node_label, _) => {
                if node_label == label {
                    return true;
                }
            }
            Tree::Leaf(lines) => {
                if lines.iter().any(|line| line.contains(label)) {
                    return true;
                }
            }
        }

        if let Tree::Node(_, children) = self {
            for (index, child) in children.iter().enumerate() {
                path.push(index);
                if child.find_path(label, path) {
                    return true;
                }
                path.pop();
            }
        }

        false
    }

    /// Sorts children at each level using the given comparison function.
    ///
    /// This recursively sorts all children throughout the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    /// use std::cmp::Ordering;
    ///
    /// let mut tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["z".to_string()]),
    ///     Tree::Leaf(vec!["a".to_string()]),
    /// ]);
    /// let mut compare = |a: &Tree, b: &Tree| {
    ///     match (a, b) {
    ///         (Tree::Leaf(lines_a), Tree::Leaf(lines_b)) => {
    ///             lines_a[0].cmp(&lines_b[0])
    ///         }
    ///         _ => Ordering::Equal,
    ///     }
    /// };
    /// tree.sort_children(&mut compare);
    /// ```
    pub fn sort_children<F>(&mut self, compare: &mut F)
    where
        F: FnMut(&Tree, &Tree) -> std::cmp::Ordering,
    {
        if let Tree::Node(_, children) = self {
            // Sort children at this level
            children.sort_by(&mut *compare);

            // Recursively sort children's children
            for child in children.iter_mut() {
                child.sort_children(compare);
            }
        }
    }

    /// Sorts children alphabetically by label (for nodes) or first line (for leaves).
    ///
    /// This recursively sorts all children throughout the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let mut tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("z".to_string(), vec![]),
    ///     Tree::Node("a".to_string(), vec![]),
    /// ]);
    /// tree.sort_by_label();
    /// ```
    pub fn sort_by_label(&mut self) {
        let mut compare = |a: &Tree, b: &Tree| {
            let label_a = match a {
                Tree::Node(label, _) => label.as_str(),
                Tree::Leaf(lines) => lines.first().map(|s| s.as_str()).unwrap_or(""),
            };
            let label_b = match b {
                Tree::Node(label, _) => label.as_str(),
                Tree::Leaf(lines) => lines.first().map(|s| s.as_str()).unwrap_or(""),
            };
            label_a.cmp(label_b)
        };
        self.sort_children(&mut compare);
    }

    /// Sorts children by depth, with the deepest first or last.
    ///
    /// This recursively sorts all children throughout the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let mut tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["shallow".to_string()]),
    ///     Tree::Node("deep".to_string(), vec![
    ///         Tree::Leaf(vec!["deep_leaf".to_string()])
    ///     ]),
    /// ]);
    /// tree.sort_by_depth(true); // deepest first
    /// ```
    pub fn sort_by_depth(&mut self, deepest_first: bool) {
        let mut compare = |a: &Tree, b: &Tree| {
            let depth_a = a.depth();
            let depth_b = b.depth();
            if deepest_first {
                depth_b.cmp(&depth_a)
            } else {
                depth_a.cmp(&depth_b)
            }
        };
        self.sort_children(&mut compare);
    }

    /// Validates the tree structure.
    ///
    /// Returns `Ok(())` if the tree is valid, or an error message if invalid.
    /// Currently checks for basic structural validity (no cycles would require
    /// more complex checking which is not needed for this tree structure).
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// assert!(tree.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        // For the current tree structure, all trees are valid
        // (no cycles possible with this structure)
        // This method exists for future extensibility
        Ok(())
    }

    /// Checks if the tree is valid.
    ///
    /// Returns `true` if valid, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// assert!(tree.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

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

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tree::Node(label, _) => write!(f, "Node({label})"),
            Tree::Leaf(lines) => {
                if lines.len() == 1 {
                    write!(f, "Leaf({})", lines[0])
                } else {
                    write!(f, "Leaf([{} lines])", lines.len())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Tree::new_node("test");
        assert!(node.is_node());
        assert_eq!(node.label(), Some("test"));
        assert_eq!(node.child_count(), Some(0));
    }

    #[test]
    fn test_new_leaf() {
        let leaf = Tree::new_leaf("line");
        assert!(leaf.is_leaf());
        assert_eq!(leaf.lines(), Some(&["line".to_string()][..]));
    }

    #[test]
    fn test_add_child() {
        let mut node = Tree::new_node("root");
        node.add_child(Tree::new_leaf("child"));
        assert_eq!(node.child_count(), Some(1));
    }

    #[test]
    fn test_add_child_to_leaf() {
        let mut leaf = Tree::new_leaf("leaf");
        assert!(leaf.add_child(Tree::new_leaf("child")).is_none());
    }

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

    #[test]
    fn test_find_node() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![Tree::Leaf(vec!["item".to_string()])],
            )],
        );
        let found = tree.find_node("child");
        assert!(found.is_some());
        assert_eq!(found.unwrap().label(), Some("child"));

        let not_found = tree.find_node("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_find_all_nodes() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node("child".to_string(), vec![]),
                Tree::Node("child".to_string(), vec![]),
                Tree::Node("other".to_string(), vec![]),
            ],
        );
        let found = tree.find_all_nodes("child");
        assert_eq!(found.len(), 2);

        let found_root = tree.find_all_nodes("root");
        assert_eq!(found_root.len(), 1);
    }

    #[test]
    fn test_find_leaf() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let found = tree.find_leaf("item");
        assert!(found.is_some());
        assert!(found.unwrap().is_leaf());

        let not_found = tree.find_leaf("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_contains() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        assert!(tree.contains("root"));
        assert!(tree.contains("item"));
        assert!(!tree.contains("nonexistent"));
    }

    #[test]
    fn test_path_to() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![Tree::Leaf(vec!["item".to_string()])],
            )],
        );
        let path = tree.path_to("child");
        assert_eq!(path, Some(vec![0]));

        let path_to_item = tree.path_to("item");
        assert_eq!(path_to_item, Some(vec![0, 0]));

        let path_not_found = tree.path_to("nonexistent");
        assert_eq!(path_not_found, None);
    }

    #[test]
    fn test_sort_children() {
        use std::cmp::Ordering;
        let mut tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["z".to_string()]),
                Tree::Leaf(vec!["a".to_string()]),
            ],
        );
        let mut compare = |a: &Tree, b: &Tree| match (a, b) {
            (Tree::Leaf(lines_a), Tree::Leaf(lines_b)) => lines_a[0].cmp(&lines_b[0]),
            _ => Ordering::Equal,
        };
        tree.sort_children(&mut compare);
        if let Tree::Node(_, children) = &tree
            && let Tree::Leaf(lines) = &children[0]
        {
            assert_eq!(lines[0], "a");
        }
    }

    #[test]
    fn test_sort_by_label() {
        let mut tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node("z".to_string(), vec![]),
                Tree::Node("a".to_string(), vec![]),
            ],
        );
        tree.sort_by_label();
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children[0].label(), Some("a"));
            assert_eq!(children[1].label(), Some("z"));
        }
    }

    #[test]
    fn test_sort_by_depth() {
        let mut tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["shallow".to_string()]),
                Tree::Node(
                    "deep".to_string(),
                    vec![Tree::Leaf(vec!["deep_leaf".to_string()])],
                ),
            ],
        );
        tree.sort_by_depth(true); // deepest first
        if let Tree::Node(_, children) = &tree {
            assert!(children[0].is_node());
            assert!(children[1].is_leaf());
        }
    }
}
