//! TOML serialization support for Tree.

use crate::tree::Tree;

impl Tree {
    /// Deserializes a tree from TOML using serde.
    ///
    /// Requires the `serde-toml` feature.
    ///
    /// This method uses serde for deserialization, preserving the exact Tree structure.
    /// For visualizing arbitrary TOML files as trees, use `Tree::from_toml()` in the `toml` module.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// // Create a tree and serialize it
    /// let original = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let toml = original.to_toml().unwrap();
    ///
    /// // Deserialize it back
    /// let tree = Tree::from_toml(&toml).unwrap();
    /// assert_eq!(original, tree);
    /// ```
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Serializes the tree to TOML using serde.
    ///
    /// Requires the `serde-toml` feature.
    ///
    /// This method uses serde for serialization, preserving the exact Tree structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let toml = tree.to_toml().unwrap();
    /// ```
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    /// Serializes the tree to pretty-printed TOML using serde.
    ///
    /// Requires the `serde-toml` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let toml = tree.to_toml_pretty().unwrap();
    /// ```
    pub fn to_toml_pretty(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_roundtrip() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let toml = tree.to_toml().unwrap();
        let deserialized = Tree::from_toml(&toml).unwrap();
        assert_eq!(tree, deserialized);
    }
}
