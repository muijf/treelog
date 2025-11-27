//! XML/HTML DOM tree building using roxmltree.

use crate::tree::Tree;
use std::path::Path;

impl Tree {
    /// Builds a tree from an XML/HTML string.
    ///
    /// Requires the `roxmltree` feature.
    ///
    /// XML/HTML elements become nodes, and text content becomes leaves.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let xml = r#"<root><child>text</child></root>"#;
    /// let tree = Tree::from_xml(xml).unwrap();
    /// ```
    #[cfg(feature = "roxmltree")]
    pub fn from_xml(xml_str: &str) -> Result<Self, roxmltree::Error> {
        let doc = roxmltree::Document::parse(xml_str)?;
        let root = doc.root_element();
        Ok(Self::from_xml_node(&root))
    }

    /// Builds a tree from an XML/HTML file.
    ///
    /// Requires the `roxmltree` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    ///
    /// let tree = Tree::from_xml_file("example.xml").unwrap();
    /// ```
    #[cfg(feature = "roxmltree")]
    pub fn from_xml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Self::from_xml(&content).map_err(|e| e.into())
    }

    #[cfg(feature = "roxmltree")]
    fn from_xml_node(node: &roxmltree::Node) -> Self {
        let mut label_parts = Vec::new();
        label_parts.push(node.tag_name().name().to_string());

        // Add attributes to the label
        let attrs: Vec<String> = node
            .attributes()
            .map(|attr| format!("{}={}", attr.name(), attr.value()))
            .collect();
        if !attrs.is_empty() {
            label_parts.push(format!("[{}]", attrs.join(", ")));
        }

        let label = label_parts.join(" ");

        let mut children = Vec::new();

        // Add text content if present
        for child in node.children() {
            if child.is_text() {
                let text = child.text().unwrap_or("").trim();
                if !text.is_empty() {
                    children.push(Tree::new_leaf(format!("text: {}", text)));
                }
            } else if child.is_element() {
                children.push(Self::from_xml_node(&child));
            }
        }

        if children.is_empty() {
            Tree::new_leaf(label)
        } else {
            Tree::Node(label, children)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "roxmltree")]
    #[test]
    fn test_from_xml() {
        let xml = r#"<root><child>text</child></root>"#;
        let tree = Tree::from_xml(xml);
        assert!(tree.is_ok());
        let tree = tree.unwrap();
        assert!(tree.is_node());
    }

    #[cfg(feature = "roxmltree")]
    #[test]
    fn test_from_xml_with_attributes() {
        let xml = r#"<root id="1"><child class="test">text</child></root>"#;
        let tree = Tree::from_xml(xml);
        assert!(tree.is_ok());
        let tree = tree.unwrap();
        assert!(tree.is_node());
    }
}
