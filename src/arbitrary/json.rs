//! JSON arbitrary serialization support for Tree.

use crate::tree::Tree;

impl Tree {
    /// Deserializes arbitrary JSON data into a tree structure.
    ///
    /// Requires the `arbitrary-json` feature.
    ///
    /// This function can parse any JSON file and convert it to a Tree representation,
    /// where objects become nodes and values become leaves.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let json_str = r#"
    /// {
    ///   "package": {
    ///     "name": "treelog",
    ///     "version": "0.0.4"
    ///   }
    /// }
    /// "#;
    /// let tree = Tree::from_arbitrary_json(json_str).unwrap();
    /// ```
    pub fn from_arbitrary_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        Ok(Self::from_json_value(&value))
    }

    // Helper functions for JSON conversion

    fn from_json_value(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::String(s) => Tree::new_leaf(format!("\"{}\"", s)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Tree::new_leaf(i.to_string())
                } else if let Some(f) = n.as_f64() {
                    Tree::new_leaf(f.to_string())
                } else {
                    Tree::new_leaf(n.to_string())
                }
            }
            serde_json::Value::Bool(b) => Tree::new_leaf(b.to_string()),
            serde_json::Value::Null => Tree::new_leaf("null".to_string()),
            serde_json::Value::Array(arr) => {
                let children: Vec<Tree> = arr
                    .iter()
                    .enumerate()
                    .map(|(idx, val)| {
                        let child = Self::from_json_value(val);
                        Tree::Node(format!("[{}]", idx), vec![child])
                    })
                    .collect();
                if children.is_empty() {
                    Tree::new_leaf("[]")
                } else {
                    Tree::Node("array".to_string(), children)
                }
            }
            serde_json::Value::Object(obj) => {
                let children: Vec<Tree> = obj
                    .iter()
                    .map(|(key, val)| {
                        let child = Self::from_json_value(val);
                        if child.is_leaf() {
                            let leaf_lines = child.lines().unwrap();
                            if leaf_lines.len() == 1 {
                                Tree::new_leaf(format!("\"{}\": {}", key, leaf_lines[0]))
                            } else {
                                Tree::Node(key.clone(), vec![child])
                            }
                        } else {
                            Tree::Node(key.clone(), vec![child])
                        }
                    })
                    .collect();
                if children.is_empty() {
                    Tree::new_leaf("{}")
                } else {
                    Tree::Node("object".to_string(), children)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_arbitrary_json() {
        let json_str = r#"
{
  "package": {
    "name": "treelog",
    "version": "0.0.4"
  }
}
"#;
        let tree = Tree::from_arbitrary_json(json_str);
        assert!(tree.is_ok());
        let tree = tree.unwrap();
        assert!(tree.is_node());
    }

    #[test]
    fn test_from_arbitrary_json_array() {
        let json_str = r#"{"dependencies": ["serde", "toml"]}"#;
        let tree = Tree::from_arbitrary_json(json_str);
        assert!(tree.is_ok());
    }
}
