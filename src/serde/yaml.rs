//! YAML serialization support for Tree.

use crate::tree::Tree;

impl Tree {
    /// Deserializes a tree from YAML.
    ///
    /// Requires the `serde-yaml` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// // Create a tree and serialize it
    /// let original = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let yaml = original.to_yaml().unwrap();
    ///
    /// // Deserialize it back
    /// let tree = Tree::from_yaml(&yaml).unwrap();
    /// assert_eq!(original, tree);
    /// ```
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /// Serializes the tree to YAML.
    ///
    /// Requires the `serde-yaml` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let yaml = tree.to_yaml().unwrap();
    /// ```
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_roundtrip() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let yaml = tree.to_yaml().unwrap();
        let deserialized = Tree::from_yaml(&yaml).unwrap();
        assert_eq!(tree, deserialized);
    }
}
