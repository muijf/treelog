//! Tree sorting operations.

use crate::tree::Tree;

impl Tree {
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
    ///
    /// Note: This method requires the `stats` feature to be enabled.
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
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[cfg(feature = "stats")]
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
