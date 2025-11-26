//! Tree comparison operations.

use crate::tree::Tree;

/// Represents a difference between two trees.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeDiff {
    /// A node/leaf exists in the first tree but not in the second
    OnlyInFirst { path: Vec<usize>, content: String },
    /// A node/leaf exists in the second tree but not in the first
    OnlyInSecond { path: Vec<usize>, content: String },
    /// A node/leaf exists in both but has different content
    DifferentContent {
        path: Vec<usize>,
        first: String,
        second: String,
    },
}

impl Tree {
    /// Compares the structure of two trees, ignoring labels and content.
    ///
    /// Returns `true` if the trees have the same structure (same number of
    /// children at each level, same node/leaf types), `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree1 = Tree::Node("root1".to_string(), vec![
    ///     Tree::Leaf(vec!["a".to_string()])
    /// ]);
    /// let tree2 = Tree::Node("root2".to_string(), vec![
    ///     Tree::Leaf(vec!["b".to_string()])
    /// ]);
    /// assert!(tree1.eq_structure(&tree2));
    /// ```
    pub fn eq_structure(&self, other: &Tree) -> bool {
        match (self, other) {
            (Tree::Node(_, children1), Tree::Node(_, children2)) => {
                if children1.len() != children2.len() {
                    return false;
                }
                children1
                    .iter()
                    .zip(children2.iter())
                    .all(|(c1, c2)| c1.eq_structure(c2))
            }
            (Tree::Leaf(_), Tree::Leaf(_)) => true,
            _ => false,
        }
    }

    /// Computes the differences between two trees.
    ///
    /// Returns a vector of `TreeDiff` entries describing all differences.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree1 = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["a".to_string()])
    /// ]);
    /// let tree2 = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["b".to_string()])
    /// ]);
    /// let diffs = tree1.diff(&tree2);
    /// ```
    pub fn diff(&self, other: &Tree) -> Vec<TreeDiff> {
        let mut diffs = Vec::new();
        self.diff_recursive(other, &mut diffs, &mut Vec::new());
        diffs
    }

    fn diff_recursive(&self, other: &Tree, diffs: &mut Vec<TreeDiff>, path: &mut Vec<usize>) {
        match (self, other) {
            (Tree::Node(label1, children1), Tree::Node(label2, children2)) => {
                if label1 != label2 {
                    diffs.push(TreeDiff::DifferentContent {
                        path: path.clone(),
                        first: label1.clone(),
                        second: label2.clone(),
                    });
                }

                // Compare children
                let max_len = children1.len().max(children2.len());
                for i in 0..max_len {
                    path.push(i);
                    match (children1.get(i), children2.get(i)) {
                        (Some(c1), Some(c2)) => {
                            c1.diff_recursive(c2, diffs, path);
                        }
                        (Some(c1), None) => {
                            let content = match c1 {
                                Tree::Node(label, _) => label.clone(),
                                Tree::Leaf(lines) => lines.first().cloned().unwrap_or_default(),
                            };
                            diffs.push(TreeDiff::OnlyInFirst {
                                path: path.clone(),
                                content,
                            });
                        }
                        (None, Some(c2)) => {
                            let content = match c2 {
                                Tree::Node(label, _) => label.clone(),
                                Tree::Leaf(lines) => lines.first().cloned().unwrap_or_default(),
                            };
                            diffs.push(TreeDiff::OnlyInSecond {
                                path: path.clone(),
                                content,
                            });
                        }
                        (None, None) => {}
                    }
                    path.pop();
                }
            }
            (Tree::Leaf(lines1), Tree::Leaf(lines2)) => {
                if lines1 != lines2 {
                    let first = lines1.first().cloned().unwrap_or_default();
                    let second = lines2.first().cloned().unwrap_or_default();
                    diffs.push(TreeDiff::DifferentContent {
                        path: path.clone(),
                        first,
                        second,
                    });
                }
            }
            (Tree::Node(label, _), Tree::Leaf(lines)) => {
                let second = lines.first().cloned().unwrap_or_default();
                diffs.push(TreeDiff::DifferentContent {
                    path: path.clone(),
                    first: label.clone(),
                    second,
                });
            }
            (Tree::Leaf(lines), Tree::Node(label, _)) => {
                let first = lines.first().cloned().unwrap_or_default();
                diffs.push(TreeDiff::DifferentContent {
                    path: path.clone(),
                    first,
                    second: label.clone(),
                });
            }
        }
    }

    /// Checks if this tree is a subtree of another tree.
    ///
    /// Returns `true` if this tree structure and content appears as a
    /// subtree within the other tree, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let subtree = Tree::Node("child".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     subtree.clone(),
    ///     Tree::Leaf(vec!["other".to_string()])
    /// ]);
    /// assert!(subtree.is_subtree_of(&tree));
    /// ```
    pub fn is_subtree_of(&self, other: &Tree) -> bool {
        // Check if this tree matches the other tree
        if self == other {
            return true;
        }

        // Check if this tree is a subtree of any child
        if let Tree::Node(_, children) = other {
            for child in children {
                if self.is_subtree_of(child) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_structure() {
        let tree1 = Tree::Node("root1".to_string(), vec![Tree::Leaf(vec!["a".to_string()])]);
        let tree2 = Tree::Node("root2".to_string(), vec![Tree::Leaf(vec!["b".to_string()])]);
        assert!(tree1.eq_structure(&tree2));

        let tree3 = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["a".to_string()]),
                Tree::Leaf(vec!["b".to_string()]),
            ],
        );
        assert!(!tree1.eq_structure(&tree3));
    }

    #[test]
    fn test_diff() {
        let tree1 = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["a".to_string()])]);
        let tree2 = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["b".to_string()])]);
        let diffs = tree1.diff(&tree2);
        assert!(!diffs.is_empty());
    }

    #[test]
    fn test_is_subtree_of() {
        let subtree = Tree::Node(
            "child".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let tree = Tree::Node(
            "root".to_string(),
            vec![subtree.clone(), Tree::Leaf(vec!["other".to_string()])],
        );
        assert!(subtree.is_subtree_of(&tree));

        let not_subtree = Tree::Node(
            "nonexistent".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        assert!(!not_subtree.is_subtree_of(&tree));
    }
}
