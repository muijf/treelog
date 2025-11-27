//! YAML arbitrary serialization support for Tree.

use crate::tree::Tree;

impl Tree {
    /// Deserializes arbitrary YAML data into a tree structure.
    ///
    /// Requires the `arbitrary-yaml` feature.
    ///
    /// This function can parse any YAML file and convert it to a Tree representation,
    /// where maps become nodes and values become leaves.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let yaml_str = r#"
    /// package:
    ///   name: treelog
    ///   version: 0.0.4
    /// "#;
    /// let tree = Tree::from_arbitrary_yaml(yaml_str).unwrap();
    /// ```
    pub fn from_arbitrary_yaml(yaml_str: &str) -> Result<Self, serde_yaml::Error> {
        let value: serde_yaml::Value = serde_yaml::from_str(yaml_str)?;
        Ok(Self::from_yaml_value(&value))
    }

    // Helper functions for YAML conversion

    fn from_yaml_value(value: &serde_yaml::Value) -> Self {
        match value {
            serde_yaml::Value::String(s) => Tree::new_leaf(format!("\"{}\"", s)),
            serde_yaml::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Tree::new_leaf(i.to_string())
                } else if let Some(f) = n.as_f64() {
                    Tree::new_leaf(f.to_string())
                } else {
                    Tree::new_leaf(n.to_string())
                }
            }
            serde_yaml::Value::Bool(b) => Tree::new_leaf(b.to_string()),
            serde_yaml::Value::Null => Tree::new_leaf("null".to_string()),
            serde_yaml::Value::Sequence(seq) => {
                let children: Vec<Tree> = seq
                    .iter()
                    .enumerate()
                    .map(|(idx, val)| {
                        let child = Self::from_yaml_value(val);
                        Tree::Node(format!("[{}]", idx), vec![child])
                    })
                    .collect();
                if children.is_empty() {
                    Tree::new_leaf("[]")
                } else {
                    Tree::Node("array".to_string(), children)
                }
            }
            serde_yaml::Value::Mapping(map) => {
                let children: Vec<Tree> = map
                    .iter()
                    .map(|(key, val)| {
                        let key_str = match key {
                            serde_yaml::Value::String(s) => s.clone(),
                            _ => format!("{:?}", key),
                        };
                        let child = Self::from_yaml_value(val);
                        if child.is_leaf() {
                            let leaf_lines = child.lines().unwrap();
                            if leaf_lines.len() == 1 {
                                Tree::new_leaf(format!("{}: {}", key_str, leaf_lines[0]))
                            } else {
                                Tree::Node(key_str, vec![child])
                            }
                        } else {
                            Tree::Node(key_str, vec![child])
                        }
                    })
                    .collect();
                if children.is_empty() {
                    Tree::new_leaf("{}")
                } else {
                    Tree::Node("object".to_string(), children)
                }
            }
            serde_yaml::Value::Tagged(tagged) => {
                // Recursively process the inner value
                Self::from_yaml_value(&tagged.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_arbitrary_yaml() {
        let yaml_str = r#"
package:
  name: treelog
  version: 0.0.4
"#;
        let tree = Tree::from_arbitrary_yaml(yaml_str);
        assert!(tree.is_ok());
        let tree = tree.unwrap();
        assert!(tree.is_node());
    }

    #[test]
    fn test_from_arbitrary_yaml_array() {
        let yaml_str = r#"
dependencies:
  - serde
  - toml
"#;
        let tree = Tree::from_arbitrary_yaml(yaml_str);
        assert!(tree.is_ok());
    }
}
