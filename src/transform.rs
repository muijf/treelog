//! Tree transformation operations.

use crate::tree::Tree;

/// Extension methods for Tree that provide transformation operations.
impl Tree {
    /// Transforms all node labels using the given function.
    ///
    /// Returns a new tree with transformed labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("child".to_string(), vec![])
    /// ]);
    /// let transformed = tree.map_nodes(|label| format!("[{}]", label));
    /// ```
    pub fn map_nodes<F>(&self, f: F) -> Tree
    where
        F: Fn(&str) -> String,
    {
        Self::map_nodes_impl(self, &f)
    }

    fn map_nodes_impl<F>(tree: &Tree, f: &F) -> Tree
    where
        F: Fn(&str) -> String,
    {
        match tree {
            Tree::Node(label, children) => {
                let new_label = f(label);
                let new_children = children
                    .iter()
                    .map(|child| Self::map_nodes_impl(child, f))
                    .collect();
                Tree::Node(new_label, new_children)
            }
            Tree::Leaf(lines) => Tree::Leaf(lines.clone()),
        }
    }

    /// Transforms all leaf lines using the given function.
    ///
    /// Returns a new tree with transformed leaf lines.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// let transformed = tree.map_leaves(|line| format!("- {}", line));
    /// ```
    pub fn map_leaves<F>(&self, f: F) -> Tree
    where
        F: Fn(&str) -> String,
    {
        Self::map_leaves_impl(self, &f)
    }

    fn map_leaves_impl<F>(tree: &Tree, f: &F) -> Tree
    where
        F: Fn(&str) -> String,
    {
        match tree {
            Tree::Node(label, children) => {
                let new_children = children
                    .iter()
                    .map(|child| Self::map_leaves_impl(child, f))
                    .collect();
                Tree::Node(label.clone(), new_children)
            }
            Tree::Leaf(lines) => {
                let new_lines = lines.iter().map(|line| f(line)).collect();
                Tree::Leaf(new_lines)
            }
        }
    }

    /// Filters the tree structure, keeping only nodes/leaves that match the predicate.
    ///
    /// If a node's children are all filtered out, the node itself is also removed.
    /// Returns `Some(Tree)` if the tree or any of its descendants match, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["keep".to_string()]),
    ///     Tree::Leaf(vec!["remove".to_string()])
    /// ]);
    /// let filtered = tree.filter(|t| {
    ///     match t {
    ///         Tree::Leaf(lines) => lines.iter().any(|l| l.contains("keep")),
    ///         Tree::Node(_, _) => true,
    ///     }
    /// });
    /// ```
    pub fn filter<F>(&self, predicate: F) -> Option<Tree>
    where
        F: Fn(&Tree) -> bool,
    {
        Self::filter_impl(self, &predicate)
    }

    fn filter_impl<F>(tree: &Tree, predicate: &F) -> Option<Tree>
    where
        F: Fn(&Tree) -> bool,
    {
        match tree {
            Tree::Node(label, children) => {
                let filtered_children: Vec<Tree> = children
                    .iter()
                    .filter_map(|child| Self::filter_impl(child, predicate))
                    .collect();

                // If this node matches or has matching children, keep it
                if predicate(tree) || !filtered_children.is_empty() {
                    Some(Tree::Node(label.clone(), filtered_children))
                } else {
                    None
                }
            }
            Tree::Leaf(lines) => {
                if predicate(tree) {
                    Some(Tree::Leaf(lines.clone()))
                } else {
                    None
                }
            }
        }
    }

    /// Prunes the tree by removing nodes/leaves that match the predicate.
    ///
    /// This is the inverse of `filter` - it removes matching items instead of keeping them.
    /// Returns `Some(Tree)` if the tree should be kept, `None` if it should be pruned.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["keep".to_string()]),
    ///     Tree::Leaf(vec!["remove".to_string()])
    /// ]);
    /// let pruned = tree.prune(|t| {
    ///     match t {
    ///         Tree::Leaf(lines) => lines.iter().any(|l| l.contains("remove")),
    ///         Tree::Node(_, _) => false,
    ///     }
    /// });
    /// ```
    pub fn prune<F>(&self, predicate: F) -> Option<Tree>
    where
        F: Fn(&Tree) -> bool,
    {
        // Prune is the inverse of filter
        self.filter(|t| !predicate(t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_nodes() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Node("child".to_string(), vec![])],
        );
        let transformed = tree.map_nodes(|label| format!("[{label}]"));
        assert_eq!(transformed.label(), Some("[root]"));
        if let Tree::Node(_, children) = &transformed {
            assert_eq!(children[0].label(), Some("[child]"));
        }
    }

    #[test]
    fn test_map_leaves() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let transformed = tree.map_leaves(|line| format!("- {line}"));
        if let Tree::Node(_, children) = &transformed {
            if let Tree::Leaf(lines) = &children[0] {
                assert_eq!(lines[0], "- item");
            }
        }
    }

    #[test]
    fn test_filter() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["keep".to_string()]),
                Tree::Leaf(vec!["remove".to_string()]),
            ],
        );
        let filtered = tree.filter(|t| match t {
            Tree::Leaf(lines) => lines.iter().any(|l| l.contains("keep")),
            Tree::Node(_, _) => true,
        });
        assert!(filtered.is_some());
        if let Tree::Node(_, children) = filtered.unwrap() {
            assert_eq!(children.len(), 1);
        }
    }

    #[test]
    fn test_prune() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["keep".to_string()]),
                Tree::Leaf(vec!["remove".to_string()]),
            ],
        );
        let pruned = tree.prune(|t| match t {
            Tree::Leaf(lines) => lines.iter().any(|l| l.contains("remove")),
            Tree::Node(_, _) => false,
        });
        assert!(pruned.is_some());
        if let Tree::Node(_, children) = pruned.unwrap() {
            assert_eq!(children.len(), 1);
        }
    }
}
