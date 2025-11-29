//! Tree search and find operations.

use crate::tree::Tree;

impl Tree {
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
