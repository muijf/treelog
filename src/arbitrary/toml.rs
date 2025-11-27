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

    /// Serializes the tree to TOML format (best-effort conversion).
    ///
    /// Requires the `arbitrary-toml` feature.
    ///
    /// This function attempts to convert a Tree back to TOML format. The conversion
    /// may be lossy as Tree doesn't preserve all TOML structure details.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("package".to_string(), vec![
    ///     Tree::Leaf(vec!["name = \"treelog\"".to_string()])
    /// ]);
    /// let toml = tree.to_arbitrary_toml().unwrap();
    /// ```
    pub fn to_arbitrary_toml(&self) -> Result<String, toml::ser::Error> {
        let value = self.to_toml_value()?;
        toml::to_string(&value)
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

    fn to_toml_value(&self) -> Result<toml::Value, toml::ser::Error> {
        match self {
            Tree::Leaf(lines) => {
                if lines.is_empty() {
                    Ok(toml::Value::String(String::new()))
                } else {
                    // Try to parse the first line as a value
                    let first_line = lines[0].trim();
                    // Try to parse as different types
                    if let Ok(i) = first_line.parse::<i64>() {
                        Ok(toml::Value::Integer(i))
                    } else if let Ok(f) = first_line.parse::<f64>() {
                        Ok(toml::Value::Float(f))
                    } else if let Ok(b) = first_line.parse::<bool>() {
                        Ok(toml::Value::Boolean(b))
                    } else if first_line.starts_with('"') && first_line.ends_with('"') {
                        // Remove quotes
                        let s = first_line[1..first_line.len() - 1].to_string();
                        Ok(toml::Value::String(s))
                    } else {
                        // Default to string
                        Ok(toml::Value::String(first_line.to_string()))
                    }
                }
            }
            Tree::Node(label, children) => {
                if label == "array" || label.starts_with('[') {
                    // This looks like an array
                    let mut arr = Vec::new();
                    for child in children {
                        // Extract value from child nodes that have index labels
                        let value = if let Tree::Node(_, grand_children) = child {
                            if grand_children.len() == 1 {
                                grand_children[0].to_toml_value()?
                            } else {
                                child.to_toml_value()?
                            }
                        } else {
                            child.to_toml_value()?
                        };
                        arr.push(value);
                    }
                    Ok(toml::Value::Array(arr))
                } else if label == "table"
                    || children.iter().any(|c| {
                        if let Tree::Leaf(lines) = c {
                            lines.iter().any(|l| l.contains('='))
                        } else {
                            false
                        }
                    })
                {
                    // This looks like a table/object
                    let mut table = toml::map::Map::new();
                    for child in children {
                        #[allow(clippy::collapsible_if)]
                        if let Tree::Leaf(lines) = child {
                            // Parse "key = value" format
                            if let Some(line) = lines.first() {
                                if let Some((key, value_str)) = line.split_once('=') {
                                    let key = key.trim();
                                    let value_str = value_str.trim();
                                    // Try to parse the value
                                    let value = if let Ok(i) = value_str.parse::<i64>() {
                                        toml::Value::Integer(i)
                                    } else if let Ok(f) = value_str.parse::<f64>() {
                                        toml::Value::Float(f)
                                    } else if let Ok(b) = value_str.parse::<bool>() {
                                        toml::Value::Boolean(b)
                                    } else if value_str.starts_with('"') && value_str.ends_with('"')
                                    {
                                        toml::Value::String(
                                            value_str[1..value_str.len() - 1].to_string(),
                                        )
                                    } else {
                                        toml::Value::String(value_str.to_string())
                                    };
                                    table.insert(key.to_string(), value);
                                    continue;
                                }
                            }
                        }
                        // If not in key=value format, use label as key
                        if let Some(key) = child.label() {
                            let value = if let Tree::Node(_, grand_children) = child {
                                if grand_children.len() == 1 {
                                    grand_children[0].to_toml_value()?
                                } else {
                                    child.to_toml_value()?
                                }
                            } else {
                                child.to_toml_value()?
                            };
                            table.insert(key.to_string(), value);
                        }
                    }
                    Ok(toml::Value::Table(table))
                } else {
                    // Default: treat as array of children
                    let arr: Result<Vec<_>, _> =
                        children.iter().map(|c| c.to_toml_value()).collect();
                    Ok(toml::Value::Array(arr?))
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
    fn test_toml_roundtrip() {
        let toml_str = r#"
[package]
name = "treelog"
version = "0.0.4"
"#;
        let tree = Tree::from_arbitrary_toml(toml_str).unwrap();
        let back = tree.to_arbitrary_toml();
        assert!(back.is_ok());
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
