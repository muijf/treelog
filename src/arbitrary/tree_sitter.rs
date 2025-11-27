//! Tree-sitter parse tree integration.

use crate::tree::Tree;

impl Tree {
    /// Builds a tree from a tree-sitter parse tree.
    ///
    /// Requires the `tree-sitter` feature.
    ///
    /// Converts a tree-sitter parse tree into a Tree structure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    /// use tree_sitter::{Parser, Language};
    ///
    /// let mut parser = Parser::new();
    /// // parser.set_language(language).unwrap();
    /// // let tree = parser.parse("source code", None).unwrap();
    /// // let tree = Tree::from_tree_sitter(&tree);
    /// ```
    #[cfg(feature = "arbitrary-tree-sitter")]
    pub fn from_tree_sitter(parse_tree: &tree_sitter::Tree) -> Self {
        let root_node = parse_tree.root_node();
        Self::from_tree_sitter_node(&root_node)
    }

    /// Builds a tree from source code using a tree-sitter language.
    ///
    /// Requires the `tree-sitter` feature.
    ///
    /// Parses the source code and converts the parse tree into a Tree structure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    /// use tree_sitter::Language;
    ///
    /// // This requires a specific language to be loaded
    /// // let tree = Tree::from_tree_sitter_language(source_code, language).unwrap();
    /// ```
    #[cfg(feature = "arbitrary-tree-sitter")]
    pub fn from_tree_sitter_language(
        source: &str,
        language: tree_sitter::Language,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language)?;
        let parse_tree = parser.parse(source, None).ok_or("Parse failed")?;
        Ok(Self::from_tree_sitter(&parse_tree))
    }

    #[cfg(feature = "arbitrary-tree-sitter")]
    fn from_tree_sitter_node(node: &tree_sitter::Node) -> Self {
        let kind = node.kind();
        let mut label = kind.to_string();

        // Add additional info if available
        if node.is_named() {
            label = format!("{} (named)", label);
        }
        if node.is_missing() {
            label = format!("{} (missing)", label);
        }
        if node.has_error() {
            label = format!("{} (error)", label);
        }

        // Add byte range info
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        if end_byte > start_byte {
            label = format!("{} [{}..{}]", label, start_byte, end_byte);
        }

        let mut children = Vec::new();

        // Process child nodes
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            children.push(Self::from_tree_sitter_node(&child));
        }

        // If no children, it's a leaf node

        if children.is_empty() {
            Tree::new_leaf(label)
        } else {
            Tree::Node(label, children)
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "arbitrary-tree-sitter")]
    #[test]
    fn test_tree_sitter_parsing() {
        // This test would require a tree-sitter language to be loaded
        // For now, we just verify the function exists
        // In a real scenario, this would need a test fixture with a language
    }
}
