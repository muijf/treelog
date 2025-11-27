//! TOML arbitrary serialization support for Tree.

use crate::tree::Tree;

impl Tree {
    /// Deserializes arbitrary TOML data into a tree structure.
    ///
    /// Requires the `arbitrary-toml` feature.
    ///
    /// This function can parse any TOML file and convert it to a Tree representation,
    /// where tables become nodes and values become leaves.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let toml_str = r#"
    /// [package]
    /// name = "treelog"
    /// version = "0.0.4"
    /// "#;
    /// let tree = Tree::from_arbitrary_toml(toml_str).unwrap();
    /// ```
    pub fn from_arbitrary_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        let value: toml::Value = toml::from_str(toml_str)?;
        Ok(Self::from_toml_value(&value))
    }

    // Helper functions for TOML conversion

    fn from_toml_value(value: &toml::Value) -> Self {
        match value {
            toml::Value::String(s) => Tree::new_leaf(format!("\"{}\"", s)),
            toml::Value::Integer(i) => Tree::new_leaf(i.to_string()),
            toml::Value::Float(f) => Tree::new_leaf(f.to_string()),
            toml::Value::Boolean(b) => Tree::new_leaf(b.to_string()),
            toml::Value::Datetime(dt) => Tree::new_leaf(dt.to_string()),
            toml::Value::Array(arr) => {
                let children: Vec<Tree> = arr
                    .iter()
                    .enumerate()
                    .map(|(idx, val)| {
                        let child = Self::from_toml_value(val);
                        Tree::Node(format!("[{}]", idx), vec![child])
                    })
                    .collect();
                if children.is_empty() {
                    Tree::new_leaf("[]")
                } else {
                    Tree::Node("array".to_string(), children)
                }
            }
            toml::Value::Table(table) => {
                let children: Vec<Tree> = table
                    .iter()
                    .map(|(key, val)| {
                        let child = Self::from_toml_value(val);
                        if child.is_leaf() {
                            // If the child is a leaf, we can merge it into the node label
                            let leaf_lines = child.lines().unwrap();
                            if leaf_lines.len() == 1 {
                                Tree::new_leaf(format!("{} = {}", key, leaf_lines[0]))
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
                    Tree::Node("table".to_string(), children)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_arbitrary_toml() {
        let toml_str = r#"
[package]
name = "treelog"
version = "0.0.4"
"#;
        let tree = Tree::from_arbitrary_toml(toml_str);
        assert!(tree.is_ok());
        let tree = tree.unwrap();
        assert!(tree.is_node());
    }

    #[test]
    fn test_from_arbitrary_toml_array() {
        let toml_str = r#"
dependencies = ["serde", "toml"]
"#;
        let tree = Tree::from_arbitrary_toml(toml_str);
        assert!(tree.is_ok());
    }
}
