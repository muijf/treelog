//! RON (Rusty Object Notation) serialization support for Tree.

use crate::tree::Tree;

impl Tree {
    /// Deserializes a tree from RON.
    ///
    /// Requires the `serde-ron` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// // Create a tree and serialize it
    /// let original = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let ron = original.to_ron().unwrap();
    ///
    /// // Deserialize it back
    /// let tree = Tree::from_ron(&ron).unwrap();
    /// assert_eq!(original, tree);
    /// ```
    pub fn from_ron(ron_str: &str) -> Result<Self, ron::de::SpannedError> {
        ron::from_str(ron_str)
    }

    /// Serializes the tree to RON.
    ///
    /// Requires the `serde-ron` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let ron = tree.to_ron().unwrap();
    /// ```
    pub fn to_ron(&self) -> Result<String, ron::error::Error> {
        ron::to_string(self)
    }

    /// Serializes the tree to pretty-printed RON.
    ///
    /// Requires the `serde-ron` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let ron = tree.to_ron_pretty().unwrap();
    /// ```
    pub fn to_ron_pretty(&self) -> Result<String, ron::error::Error> {
        let pretty = ron::ser::PrettyConfig::default();
        ron::ser::to_string_pretty(self, pretty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ron_roundtrip() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["item1".to_string()]),
                Tree::Node(
                    "sub".to_string(),
                    vec![Tree::Leaf(vec!["subitem".to_string()])],
                ),
            ],
        );
        let ron = tree.to_ron().unwrap();
        let deserialized = Tree::from_ron(&ron).unwrap();
        assert_eq!(tree, deserialized);
    }

    #[test]
    fn test_ron_pretty() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let ron = tree.to_ron_pretty().unwrap();
        assert!(!ron.is_empty());
    }
}
