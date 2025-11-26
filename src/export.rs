//! Tree export to various formats (HTML, SVG, DOT).

use crate::tree::Tree;

impl Tree {
    /// Exports the tree as HTML with collapsible nodes.
    ///
    /// Requires the `export` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// let html = tree.to_html();
    /// ```
    pub fn to_html(&self) -> String {
        let mut html = String::from(
            r#"<div class="tree">
<style>
.tree ul { list-style: none; margin: 0; padding-left: 20px; }
.tree li { margin: 5px 0; }
.tree .node { cursor: pointer; }
.tree .node::before { content: "▶ "; }
.tree .node.expanded::before { content: "▼ "; }
.tree .children { display: none; }
.tree .node.expanded + .children { display: block; }
.tree .leaf { color: #666; }
</style>
"#,
        );
        self.to_html_recursive(&mut html, 0);
        html.push_str("</div>");
        html
    }

    fn to_html_recursive(&self, html: &mut String, depth: usize) {
        match self {
            Tree::Node(label, children) => {
                let indent = "  ".repeat(depth);
                html.push_str(&format!(
                    "{indent}<li class=\"node\" onclick=\"this.classList.toggle('expanded')\">{label}\n"
                ));
                if !children.is_empty() {
                    html.push_str(&format!("{indent}  <ul class=\"children\">\n"));
                    for child in children {
                        child.to_html_recursive(html, depth + 2);
                    }
                    html.push_str(&format!("{indent}  </ul>\n"));
                }
                html.push_str(&format!("{indent}</li>\n"));
            }
            Tree::Leaf(lines) => {
                let indent = "  ".repeat(depth);
                for line in lines {
                    html.push_str(&format!(
                        "{indent}<li class=\"leaf\">{line}</li>\n",
                        line = html_escape(line)
                    ));
                }
            }
        }
    }

    /// Exports the tree as SVG tree diagram.
    ///
    /// Requires the `export` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// let svg = tree.to_svg();
    /// ```
    pub fn to_svg(&self) -> String {
        // Simple SVG representation - in a real implementation, you'd calculate
        // proper positioning and layout
        let mut svg = String::from(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="400" height="300">
"#,
        );
        self.to_svg_recursive(&mut svg, 200.0, 20.0, 50.0);
        svg.push_str("</svg>");
        svg
    }

    fn to_svg_recursive(&self, svg: &mut String, x: f64, y: f64, y_step: f64) {
        match self {
            Tree::Node(label, children) => {
                // Draw node
                svg.push_str(&format!(
                    r#"  <rect x="{}" y="{}" width="80" height="20" fill="lightblue" stroke="black"/>
  <text x="{}" y="{}" font-size="12" text-anchor="middle" dominant-baseline="middle">{}</text>
"#,
                    x - 40.0,
                    y - 10.0,
                    x,
                    y,
                    svg_escape(label)
                ));

                // Draw children
                let child_y = y + y_step;
                for (i, child) in children.iter().enumerate() {
                    let child_x = x + (i as f64 - (children.len() as f64 - 1.0) / 2.0) * 100.0;
                    // Draw line
                    svg.push_str(&format!(
                        r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black"/>
"#,
                        x,
                        y + 10.0,
                        child_x,
                        child_y - 10.0
                    ));
                    child.to_svg_recursive(svg, child_x, child_y, y_step);
                }
            }
            Tree::Leaf(lines) => {
                let text = lines.first().map(|s| s.as_str()).unwrap_or("");
                svg.push_str(&format!(
                    r#"  <rect x="{}" y="{}" width="80" height="20" fill="lightgreen" stroke="black"/>
  <text x="{}" y="{}" font-size="12" text-anchor="middle" dominant-baseline="middle">{}</text>
"#,
                    x - 40.0,
                    y - 10.0,
                    x,
                    y,
                    svg_escape(text)
                ));
            }
        }
    }

    /// Exports the tree as Graphviz DOT format.
    ///
    /// Requires the `export` feature.
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

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn svg_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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
    fn test_to_html() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let html = tree.to_html();
        assert!(html.contains("root"));
        assert!(html.contains("item"));
    }

    #[test]
    fn test_to_svg() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let svg = tree.to_svg();
        assert!(svg.contains("root"));
        assert!(svg.contains("<svg"));
    }

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
