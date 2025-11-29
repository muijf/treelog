//! Tree path utilities for navigating and accessing tree elements by path.

use crate::tree::Tree;

/// Represents a path through a tree as a sequence of child indices.
pub type TreePath = Vec<usize>;

/// Represents a flattened tree entry with its path and content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlattenedEntry {
    /// The path from root to this entry
    pub path: TreePath,
    /// The content (label for nodes, first line for leaves)
    pub content: String,
    /// Whether this entry is a node
    pub is_node: bool,
}

impl Tree {
    /// Gets the path from the root to this specific tree node.
    ///
    /// This method is typically called on a reference obtained from traversal.
    /// Returns `None` if the node is not found in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("child".to_string(), vec![])
    /// ]);
    /// let child = &tree.children().unwrap()[0];
    /// let path = tree.get_path(child);
    /// assert_eq!(path, Some(vec![0]));
    /// ```
    pub fn get_path(&self, target: &Tree) -> Option<TreePath> {
        if std::ptr::eq(self, target) {
            return Some(Vec::new());
        }

        if let Tree::Node(_, children) = self {
            for (index, child) in children.iter().enumerate() {
                if let Some(mut path) = child.get_path(target) {
                    path.insert(0, index);
                    return Some(path);
                }
            }
        }

        None
    }

    /// Gets a node at the specified path.
    ///
    /// Returns `Some(&Tree)` if the path is valid, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("child".to_string(), vec![
    ///         Tree::Leaf(vec!["item".to_string()])
    ///     ])
    /// ]);
    /// let node = tree.get_by_path(&[0, 0]);
    /// assert!(node.is_some());
    /// ```
    pub fn get_by_path(&self, path: &[usize]) -> Option<&Tree> {
        if path.is_empty() {
            return Some(self);
        }

        if let Tree::Node(_, children) = self
            && let Some(&first) = path.first()
            && first < children.len()
        {
            return children[first].get_by_path(&path[1..]);
        }

        None
    }

    /// Gets a mutable reference to a node at the specified path.
    ///
    /// Returns `Some(&mut Tree)` if the path is valid, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let mut tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Node("child".to_string(), vec![])
    /// ]);
    /// if let Some(node) = tree.get_by_path_mut(&[0]) {
    ///     if let Tree::Node(label, _) = node {
    ///         *label = "new_label".to_string();
    ///     }
    /// }
    /// ```
    pub fn get_by_path_mut(&mut self, path: &[usize]) -> Option<&mut Tree> {
        if path.is_empty() {
            return Some(self);
        }

        if let Tree::Node(_, children) = self
            && let Some(&first) = path.first()
            && first < children.len()
        {
            return children[first].get_by_path_mut(&path[1..]);
        }

        None
    }

    /// Flattens the tree into a list of entries with their paths.
    ///
    /// Returns a vector of `FlattenedEntry` containing the path and content
    /// of each node and leaf in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// let flattened = tree.flatten();
    /// assert_eq!(flattened.len(), 2);
    /// ```
    pub fn flatten(&self) -> Vec<FlattenedEntry> {
        let mut result = Vec::new();
        self.flatten_recursive(&mut result, &mut Vec::new());
        result
    }

    fn flatten_recursive(&self, result: &mut Vec<FlattenedEntry>, path: &mut TreePath) {
        match self {
            Tree::Node(label, children) => {
                result.push(FlattenedEntry {
                    path: path.clone(),
                    content: label.clone(),
                    is_node: true,
                });
                for (index, child) in children.iter().enumerate() {
                    path.push(index);
                    child.flatten_recursive(result, path);
                    path.pop();
                }
            }
            Tree::Leaf(lines) => {
                let content = lines.first().cloned().unwrap_or_default();
                result.push(FlattenedEntry {
                    path: path.clone(),
                    content,
                    is_node: false,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![Tree::Leaf(vec!["item".to_string()])],
            )],
        );
        let child = &tree.children().unwrap()[0];
        let path = tree.get_path(child);
        assert_eq!(path, Some(vec![0]));

        let root_path = tree.get_path(&tree);
        assert_eq!(root_path, Some(vec![]));
    }

    #[test]
    fn test_get_by_path() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![Tree::Leaf(vec!["item".to_string()])],
            )],
        );
        let node = tree.get_by_path(&[0]);
        assert!(node.is_some());
        assert_eq!(node.unwrap().label(), Some("child"));

        let leaf = tree.get_by_path(&[0, 0]);
        assert!(leaf.is_some());
        assert!(leaf.unwrap().is_leaf());

        let invalid = tree.get_by_path(&[99]);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_get_by_path_mut() {
        let mut tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Node("child".to_string(), vec![])],
        );
        if let Some(Tree::Node(label, _)) = tree.get_by_path_mut(&[0]) {
            *label = "new_label".to_string();
        }
        assert_eq!(tree.get_by_path(&[0]).unwrap().label(), Some("new_label"));
    }

    #[test]
    fn test_flatten() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node(
                    "child".to_string(),
                    vec![Tree::Leaf(vec!["item".to_string()])],
                ),
                Tree::Leaf(vec!["leaf2".to_string()]),
            ],
        );
        let flattened = tree.flatten();
        assert_eq!(flattened.len(), 4);
        assert_eq!(flattened[0].content, "root");
        assert!(flattened[0].is_node);
        assert_eq!(flattened[1].content, "child");
        assert_eq!(flattened[2].content, "item");
        assert!(!flattened[2].is_node);
    }
}
