//! Core tree data structure for representing hierarchical data.

use std::fmt;

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
}
