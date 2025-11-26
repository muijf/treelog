//! High-level builder API for constructing trees.

use crate::tree::Tree;

/// A builder for constructing trees using a fluent API.
///
/// This provides a convenient way to build trees without manually
/// constructing nested `Tree::Node` and `Tree::Leaf` variants.
///
/// # Examples
///
/// ```
/// use treelog::builder::TreeBuilder;
///
/// let mut builder = TreeBuilder::new();
/// builder.node("root").leaf("item1");
/// let tree = builder.build();
/// ```
pub struct TreeBuilder {
    stack: Vec<Tree>,
}

impl TreeBuilder {
    /// Creates a new tree builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::builder::TreeBuilder;
    ///
    /// let builder = TreeBuilder::new();
    /// ```
    pub fn new() -> Self {
        TreeBuilder { stack: Vec::new() }
    }

    /// Adds a node with the given label and makes it the current context.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::builder::TreeBuilder;
    ///
    /// let mut builder = TreeBuilder::new();
    /// builder.node("root");
    /// ```
    pub fn node(&mut self, label: impl Into<String>) -> &mut Self {
        let node = Tree::new_node(label);
        self.stack.push(node);
        self
    }

    /// Adds a leaf with a single line of text.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::builder::TreeBuilder;
    ///
    /// let mut builder = TreeBuilder::new();
    /// builder.node("root").leaf("item");
    /// ```
    pub fn leaf(&mut self, line: impl Into<String>) -> &mut Self {
        if let Some(Tree::Node(_, children)) = self.stack.last_mut() {
            children.push(Tree::new_leaf(line));
        }
        self
    }

    /// Adds a leaf with multiple lines of text.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::builder::TreeBuilder;
    ///
    /// let mut builder = TreeBuilder::new();
    /// builder.node("root").leaf_lines(vec!["line1", "line2"]);
    /// ```
    pub fn leaf_lines(&mut self, lines: Vec<impl Into<String>>) -> &mut Self {
        if let Some(Tree::Node(_, children)) = self.stack.last_mut() {
            children.push(Tree::new_leaf_lines(lines));
        }
        self
    }

    /// Ends the current node context and returns to the parent.
    ///
    /// This should be called after adding children to a node to return
    /// to the parent context.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::builder::TreeBuilder;
    ///
    /// let mut builder = TreeBuilder::new();
    /// builder.node("root")
    ///     .node("child")
    ///         .leaf("item")
    ///     .end()  // Returns to "root" context
    ///     .leaf("another");
    /// ```
    pub fn end(&mut self) -> &mut Self {
        if self.stack.len() > 1 {
            let child = self.stack.pop().unwrap();
            if let Some(Tree::Node(_, children)) = self.stack.last_mut() {
                children.push(child);
            }
        }
        self
    }

    /// Builds and returns the final tree.
    ///
    /// # Panics
    ///
    /// Panics if no nodes have been added or if the builder is in an invalid state.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::builder::TreeBuilder;
    ///
    /// let mut builder = TreeBuilder::new();
    /// builder.node("root").leaf("item");
    /// let tree = builder.build();
    /// ```
    pub fn build(mut self) -> Tree {
        if self.stack.is_empty() {
            panic!("TreeBuilder: cannot build empty tree");
        }
        if self.stack.len() > 1 {
            // Close all remaining nodes
            while self.stack.len() > 1 {
                self.end();
            }
        }
        self.stack.pop().unwrap()
    }
}

impl Default for TreeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tree() {
        let mut builder = TreeBuilder::new();
        builder.node("root").leaf("item");
        let tree = builder.build();

        assert!(tree.is_node());
        assert_eq!(tree.label(), Some("root"));
    }

    #[test]
    fn test_nested_tree() {
        let mut builder = TreeBuilder::new();
        builder.node("root").node("child").leaf("item").end();
        let tree = builder.build();

        assert!(tree.is_node());
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 1);
            assert!(children[0].is_node());
        }
    }

    #[test]
    fn test_multiple_children() {
        let mut builder = TreeBuilder::new();
        builder
            .node("root")
            .leaf("item1")
            .leaf("item2")
            .node("child")
            .leaf("subitem")
            .end();
        let tree = builder.build();

        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 3);
        }
    }
}
