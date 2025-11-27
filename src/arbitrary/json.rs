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

    /// Serializes the tree to JSON format (best-effort conversion).
    ///
    /// Requires the `arbitrary-json` feature.
    ///
    /// This function attempts to convert a Tree back to JSON format. The conversion
    /// may be lossy as Tree doesn't preserve all JSON structure details.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("package".to_string(), vec![
    ///     Tree::Leaf(vec!["name: treelog".to_string()])
    /// ]);
    /// let json = tree.to_arbitrary_json().unwrap();
    /// ```
    pub fn to_arbitrary_json(&self) -> Result<String, serde_json::Error> {
        let value = self.to_json_value()?;
        serde_json::to_string(&value)
    }

    /// Serializes the tree to pretty-printed JSON format (best-effort conversion).
    ///
    /// Requires the `arbitrary-json` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("package".to_string(), vec![
    ///     Tree::Leaf(vec!["name: treelog".to_string()])
    /// ]);
    /// let json = tree.to_arbitrary_json_pretty().unwrap();
    /// ```
    pub fn to_arbitrary_json_pretty(&self) -> Result<String, serde_json::Error> {
        let value = self.to_json_value()?;
        serde_json::to_string_pretty(&value)
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

    fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        match self {
            Tree::Leaf(lines) => {
                if lines.is_empty() {
                    Ok(serde_json::Value::Null)
                } else {
                    let first_line = lines[0].trim();
                    // Try to parse as different types
                    if first_line == "null" {
                        Ok(serde_json::Value::Null)
                    } else if let Ok(i) = first_line.parse::<i64>() {
                        Ok(serde_json::Value::Number(i.into()))
                    } else if let Ok(f) = first_line.parse::<f64>() {
                        Ok(serde_json::Value::Number(
                            serde_json::Number::from_f64(f)
                                .unwrap_or_else(|| serde_json::Number::from(0)),
                        ))
                    } else if let Ok(b) = first_line.parse::<bool>() {
                        Ok(serde_json::Value::Bool(b))
                    } else if first_line.starts_with('"') && first_line.ends_with('"') {
                        Ok(serde_json::Value::String(
                            first_line[1..first_line.len() - 1].to_string(),
                        ))
                    } else {
                        Ok(serde_json::Value::String(first_line.to_string()))
                    }
                }
            }
            Tree::Node(label, children) => {
                if label == "array" || label.starts_with('[') {
                    let mut arr = Vec::new();
                    for child in children {
                        let value = if let Tree::Node(_, grand_children) = child {
                            if grand_children.len() == 1 {
                                grand_children[0].to_json_value()?
                            } else {
                                child.to_json_value()?
                            }
                        } else {
                            child.to_json_value()?
                        };
                        arr.push(value);
                    }
                    Ok(serde_json::Value::Array(arr))
                } else if label == "object"
                    || children.iter().any(|c| {
                        if let Tree::Leaf(lines) = c {
                            lines.iter().any(|l| l.contains(':'))
                        } else {
                            false
                        }
                    })
                {
                    let mut obj = serde_json::Map::new();
                    for child in children {
                        // Parse "key": value format
                        #[allow(clippy::collapsible_if)]
                        if let Tree::Leaf(lines) = child {
                            if let Some(line) = lines.first() {
                                if let Some((key_part, value_str)) = line.split_once(':') {
                                    let key = key_part.trim().trim_matches('"');
                                    let value_str = value_str.trim();
                                    let value = if value_str == "null" {
                                        serde_json::Value::Null
                                    } else if let Ok(i) = value_str.parse::<i64>() {
                                        serde_json::Value::Number(i.into())
                                    } else if let Ok(f) = value_str.parse::<f64>() {
                                        serde_json::Value::Number(
                                            serde_json::Number::from_f64(f)
                                                .unwrap_or_else(|| serde_json::Number::from(0)),
                                        )
                                    } else if let Ok(b) = value_str.parse::<bool>() {
                                        serde_json::Value::Bool(b)
                                    } else if value_str.starts_with('"') && value_str.ends_with('"')
                                    {
                                        serde_json::Value::String(
                                            value_str[1..value_str.len() - 1].to_string(),
                                        )
                                    } else {
                                        serde_json::Value::String(value_str.to_string())
                                    };
                                    obj.insert(key.to_string(), value);
                                    continue;
                                }
                            }
                        }
                        if let Some(key) = child.label() {
                            let value = if let Tree::Node(_, grand_children) = child {
                                if grand_children.len() == 1 {
                                    grand_children[0].to_json_value()?
                                } else {
                                    child.to_json_value()?
                                }
                            } else {
                                child.to_json_value()?
                            };
                            obj.insert(key.to_string(), value);
                        }
                    }
                    Ok(serde_json::Value::Object(obj))
                } else {
                    let arr: Result<Vec<_>, _> =
                        children.iter().map(|c| c.to_json_value()).collect();
                    Ok(serde_json::Value::Array(arr?))
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
    fn test_json_roundtrip() {
        let json_str = r#"{"package": {"name": "treelog", "version": "0.0.4"}}"#;
        let tree = Tree::from_arbitrary_json(json_str).unwrap();
        let back = tree.to_arbitrary_json();
        assert!(back.is_ok());
    }

    #[test]
    fn test_from_arbitrary_json_array() {
        let json_str = r#"{"dependencies": ["serde", "toml"]}"#;
        let tree = Tree::from_arbitrary_json(json_str);
        assert!(tree.is_ok());
    }
}
