//! Graphviz DOT export functionality for Tree.

use crate::tree::Tree;

impl Tree {
    /// Exports the tree as Graphviz DOT format.
    ///
    /// Requires the `export-dot` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// let dot = tree.to_dot();
    /// ```
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph Tree {\n");
        let mut node_id = 0;
        self.to_dot_recursive(&mut dot, &mut node_id, None);
        dot.push_str("}\n");
        dot
    }

    fn to_dot_recursive(&self, dot: &mut String, node_id: &mut usize, parent: Option<usize>) {
        let current_id = *node_id;
        *node_id += 1;

        match self {
            Tree::Node(label, _) => {
                dot.push_str(&format!(
                    "  node{} [label=\"{}\"];\n",
                    current_id,
                    dot_escape(label)
                ));
            }
            Tree::Leaf(lines) => {
                let text = lines.first().map(|s| s.as_str()).unwrap_or("");
                dot.push_str(&format!(
                    "  node{} [label=\"{}\", shape=box];\n",
                    current_id,
                    dot_escape(text)
                ));
            }
        }

        if let Some(parent_id) = parent {
            dot.push_str(&format!("  node{parent_id} -> node{current_id};\n"));
        }

        if let Tree::Node(_, children) = self {
            for child in children {
                child.to_dot_recursive(dot, node_id, Some(current_id));
            }
        }
    }
}

fn dot_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_dot() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let dot = tree.to_dot();
        assert!(dot.contains("root"));
        assert!(dot.contains("digraph"));
    }
}
