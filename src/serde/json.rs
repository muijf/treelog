//! JSON serialization support for Tree.

use crate::tree::Tree;

impl Tree {
    /// Deserializes a tree from JSON.
    ///
    /// Requires the `serde-json` feature.
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
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serializes the tree to JSON.
    ///
    /// Requires the `serde-json` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let json = tree.to_json().unwrap();
    /// ```
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serializes the tree to pretty-printed JSON.
    ///
    /// Requires the `serde-json` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let json = tree.to_json_pretty().unwrap();
    /// ```
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
