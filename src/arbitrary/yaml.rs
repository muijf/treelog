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

    /// Serializes the tree to YAML format (best-effort conversion).
    ///
    /// Requires the `arbitrary-yaml` feature.
    ///
    /// This function attempts to convert a Tree back to YAML format. The conversion
    /// may be lossy as Tree doesn't preserve all YAML structure details.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("package".to_string(), vec![
    ///     Tree::Leaf(vec!["name: treelog".to_string()])
    /// ]);
    /// let yaml = tree.to_arbitrary_yaml().unwrap();
    /// ```
    pub fn to_arbitrary_yaml(&self) -> Result<String, serde_yaml::Error> {
        let value = self.to_yaml_value()?;
        serde_yaml::to_string(&value)
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

    fn to_yaml_value(&self) -> Result<serde_yaml::Value, serde_yaml::Error> {
        match self {
            Tree::Leaf(lines) => {
                if lines.is_empty() {
                    Ok(serde_yaml::Value::Null)
                } else {
                    let first_line = lines[0].trim();
                    // Try to parse as different types
                    if first_line == "null" {
                        Ok(serde_yaml::Value::Null)
                    } else if let Ok(i) = first_line.parse::<i64>() {
                        Ok(serde_yaml::Value::Number(i.into()))
                    } else if let Ok(f) = first_line.parse::<f64>() {
                        // serde_yaml::Number doesn't have from_f64, use serde_yaml::to_value
                        match serde_yaml::to_value(f) {
                            Ok(serde_yaml::Value::Number(n)) => Ok(serde_yaml::Value::Number(n)),
                            _ => Ok(serde_yaml::Value::String(f.to_string())),
                        }
                    } else if let Ok(b) = first_line.parse::<bool>() {
                        Ok(serde_yaml::Value::Bool(b))
                    } else if first_line.starts_with('"') && first_line.ends_with('"') {
                        Ok(serde_yaml::Value::String(
                            first_line[1..first_line.len() - 1].to_string(),
                        ))
                    } else {
                        Ok(serde_yaml::Value::String(first_line.to_string()))
                    }
                }
            }
            Tree::Node(label, children) => {
                if label == "array" || label.starts_with('[') {
                    let mut seq = Vec::new();
                    for child in children {
                        let value = if let Tree::Node(_, grand_children) = child {
                            if grand_children.len() == 1 {
                                grand_children[0].to_yaml_value()?
                            } else {
                                child.to_yaml_value()?
                            }
                        } else {
                            child.to_yaml_value()?
                        };
                        seq.push(value);
                    }
                    Ok(serde_yaml::Value::Sequence(seq))
                } else if label == "object"
                    || children.iter().any(|c| {
                        if let Tree::Leaf(lines) = c {
                            lines.iter().any(|l| l.contains(':'))
                        } else {
                            false
                        }
                    })
                {
                    let mut map = serde_yaml::Mapping::new();
                    for child in children {
                        #[allow(clippy::collapsible_if)]
                        if let Tree::Leaf(lines) = child {
                            if let Some(line) = lines.first() {
                                if let Some((key, value_str)) = line.split_once(':') {
                                    let key = key.trim();
                                    let value_str = value_str.trim();
                                    let value = if value_str == "null" {
                                        serde_yaml::Value::Null
                                    } else if let Ok(i) = value_str.parse::<i64>() {
                                        serde_yaml::Value::Number(i.into())
                                    } else if let Ok(f) = value_str.parse::<f64>() {
                                        // serde_yaml::Number doesn't have from_f64, use serde_yaml::to_value
                                        match serde_yaml::to_value(f) {
                                            Ok(serde_yaml::Value::Number(n)) => {
                                                serde_yaml::Value::Number(n)
                                            }
                                            _ => serde_yaml::Value::String(f.to_string()),
                                        }
                                    } else if let Ok(b) = value_str.parse::<bool>() {
                                        serde_yaml::Value::Bool(b)
                                    } else if value_str.starts_with('"') && value_str.ends_with('"')
                                    {
                                        serde_yaml::Value::String(
                                            value_str[1..value_str.len() - 1].to_string(),
                                        )
                                    } else {
                                        serde_yaml::Value::String(value_str.to_string())
                                    };
                                    map.insert(serde_yaml::Value::String(key.to_string()), value);
                                    continue;
                                }
                            }
                        }
                        if let Some(key) = child.label() {
                            let value = if let Tree::Node(_, grand_children) = child {
                                if grand_children.len() == 1 {
                                    grand_children[0].to_yaml_value()?
                                } else {
                                    child.to_yaml_value()?
                                }
                            } else {
                                child.to_yaml_value()?
                            };
                            map.insert(serde_yaml::Value::String(key.to_string()), value);
                        }
                    }
                    Ok(serde_yaml::Value::Mapping(map))
                } else {
                    let seq: Result<Vec<_>, _> =
                        children.iter().map(|c| c.to_yaml_value()).collect();
                    Ok(serde_yaml::Value::Sequence(seq?))
                }
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
    fn test_yaml_roundtrip() {
        let yaml_str = r#"
package:
  name: treelog
  version: 0.0.4
"#;
        let tree = Tree::from_arbitrary_yaml(yaml_str).unwrap();
        let back = tree.to_arbitrary_yaml();
        assert!(back.is_ok());
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
