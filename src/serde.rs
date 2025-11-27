//! Serde serialization and deserialization support for Tree.

use crate::tree::Tree;

impl Tree {
    /// Deserializes a tree from JSON.
    ///
    /// Requires the `json` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// // Create a tree and serialize it
    /// let original = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let json = original.to_json().unwrap();
    ///
    /// // Deserialize it back
    /// let tree = Tree::from_json(&json).unwrap();
    /// assert_eq!(original, tree);
    /// ```
    #[cfg(feature = "json")]
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serializes the tree to JSON.
    ///
    /// Requires the `json` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let json = tree.to_json().unwrap();
    /// ```
    #[cfg(feature = "json")]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serializes the tree to pretty-printed JSON.
    ///
    /// Requires the `json` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let json = tree.to_json_pretty().unwrap();
    /// ```
    #[cfg(feature = "json")]
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes a tree from YAML.
    ///
    /// Requires the `yaml` feature.
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
    #[cfg(feature = "yaml")]
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /// Serializes the tree to YAML.
    ///
    /// Requires the `yaml` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let yaml = tree.to_yaml().unwrap();
    /// ```
    #[cfg(feature = "yaml")]
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    /// Deserializes a tree from TOML using serde.
    ///
    /// Requires both `toml` and `serde` features.
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
    #[cfg(all(feature = "toml", feature = "serde"))]
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Serializes the tree to TOML using serde.
    ///
    /// Requires both `toml` and `serde` features.
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
    #[cfg(all(feature = "toml", feature = "serde"))]
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    /// Serializes the tree to pretty-printed TOML using serde.
    ///
    /// Requires both `toml` and `serde` features.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let toml = tree.to_toml_pretty().unwrap();
    /// ```
    #[cfg(all(feature = "toml", feature = "serde"))]
    pub fn to_toml_pretty(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "json")]
    #[test]
    fn test_json_roundtrip() {
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
        let json = tree.to_json().unwrap();
        let deserialized = Tree::from_json(&json).unwrap();
        assert_eq!(tree, deserialized);
    }

    #[cfg(feature = "yaml")]
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

    #[cfg(all(feature = "toml", feature = "serde"))]
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
