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
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: Helvetica Neue, Helvetica, Arial, sans-serif; font-size: 20px; }
.tree ul { list-style: none; padding-left: 0.5em; margin-left: 0.3em; border-left: 3px solid #c0d1d1; margin-bottom: 1em; color: #212b2b; }
.tree li { list-style-type: none; margin-bottom: 0.5em; margin-top: 0.5em; }
.tree details summary { cursor: pointer; color: #4C74B9; }
.tree details summary::-webkit-details-marker { color: #4C74B9; font-size: 18px; }
.tree details[open] > summary::-webkit-details-marker { color: #2b4b82; }
.tree details[open] > summary { color: #2b4b82; }
</style>
<ul>
"#,
        );
        self.to_html_recursive(&mut html, 0);
        html.push_str("</ul></div>");
        html
    }

    fn to_html_recursive(&self, html: &mut String, depth: usize) {
        match self {
            Tree::Node(label, children) => {
                let indent = "  ".repeat(depth);
                if !children.is_empty() {
                    html.push_str(&format!(
                        "{indent}<li>\n{indent}  <details>\n{indent}    <summary>{}</summary>\n{indent}    <ul>\n",
                        html_escape(label)
                    ));
                    for child in children {
                        child.to_html_recursive(html, depth + 2);
                    }
                    html.push_str(&format!(
                        "{indent}    </ul>\n{indent}  </details>\n{indent}</li>\n"
                    ));
                } else {
                    html.push_str(&format!("{indent}<li>{}</li>\n", html_escape(label)));
                }
            }
            Tree::Leaf(lines) => {
                let indent = "  ".repeat(depth);
                for line in lines {
                    html.push_str(&format!("{indent}<li>{}</li>\n", html_escape(line)));
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
        // Calculate layout dimensions
        let mut layout = SvgLayout::new();
        layout.calculate_layout(self);

        let padding = 20.0;
        let width = layout.max_x + padding * 2.0;
        let height = layout.max_y + padding * 2.0;

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.0}\" height=\"{:.0}\">\n<style>\n  .node-text {{ fill: #4C74B9; font-family: Helvetica Neue, Helvetica, Arial, sans-serif; font-size: 20px; }}\n  .leaf-text {{ fill: #212b2b; font-family: Helvetica Neue, Helvetica, Arial, sans-serif; font-size: 20px; }}\n  .tree-line {{ stroke: #c0d1d1; stroke-width: 3; }}\n</style>\n",
            width, height
        );

        self.to_svg_recursive(&mut svg, padding, padding);
        svg.push_str("</svg>");
        svg
    }

    fn to_svg_recursive(&self, svg: &mut String, x: f64, y: f64) {
        match self {
            Tree::Node(label, children) => {
                let node_x = x;
                let node_y = y;

                // Draw text with middle baseline (centered vertically)
                svg.push_str(&format!(
                    r#"  <text x="{}" y="{}" class="node-text" dominant-baseline="middle">{}</text>
"#,
                    node_x,
                    node_y,
                    svg_escape(label)
                ));

                if !children.is_empty() {
                    let child_start_y = node_y + 30.0;
                    let mut child_y = child_start_y;
                    let vertical_line_x = x + 2.5;

                    // Calculate total height of all children (recursively)
                    let mut total_height = 0.0;
                    for child in children.iter() {
                        total_height += calculate_tree_height(child);
                    }

                    // Track the last child's y position for the vertical line
                    let mut last_child_y = child_start_y;

                    for child in children {
                        let child_x = x + 30.0;

                        // Draw horizontal line to child (at middle of character)
                        // Horizontal lines align with the middle of the text
                        svg.push_str(&format!(
                            r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="tree-line"/>
"#,
                            vertical_line_x, child_y, child_x, child_y
                        ));

                        child.to_svg_recursive(svg, child_x, child_y);

                        // Track this child's position for the vertical line
                        last_child_y = child_y;

                        // Calculate next child position using recursive height
                        child_y += calculate_tree_height(child);
                    }

                    // Draw vertical line from parent through all children
                    // Vertical line starts below the first character (below the text)
                    // Text center is at node_y, so below it is node_y + 10 (half font size)
                    if total_height > 0.0 {
                        svg.push_str(&format!(
                            r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="tree-line"/>
"#,
                            vertical_line_x,
                            node_y + 10.0,
                            vertical_line_x,
                            last_child_y
                        ));
                    }
                }
            }
            Tree::Leaf(lines) => {
                let mut leaf_y = y;
                for line in lines {
                    svg.push_str(&format!(
                        r#"  <text x="{}" y="{}" class="leaf-text" dominant-baseline="middle">{}</text>
"#,
                        x,
                        leaf_y,
                        svg_escape(line)
                    ));
                    leaf_y += 30.0;
                }
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

struct SvgLayout {
    max_x: f64,
    max_y: f64,
}

impl SvgLayout {
    fn new() -> Self {
        Self {
            max_x: 0.0,
            max_y: 0.0,
        }
    }

    fn calculate_layout(&mut self, tree: &Tree) {
        self.calculate_recursive(tree, 0.0, 0.0);
    }

    fn calculate_recursive(&mut self, tree: &Tree, x: f64, y: f64) {
        match tree {
            Tree::Node(label, children) => {
                // Track max_x for this node's label
                self.max_x = self.max_x.max(x + estimate_text_width(label));
                self.max_y = self.max_y.max(y);

                if !children.is_empty() {
                    let child_start_y = y + 30.0;
                    let mut child_y = child_start_y;

                    for child in children {
                        let child_x = x + 30.0;
                        // Recursively calculate layout for child
                        self.calculate_recursive(child, child_x, child_y);
                        // Move to next child position (exactly like rendering does)
                        child_y += calculate_tree_height(child);
                    }
                    // Track the final y position after all children
                    self.max_y = self.max_y.max(child_y);
                }
            }
            Tree::Leaf(lines) => {
                // Track max_x for leaf text
                if let Some(first_line) = lines.first() {
                    self.max_x = self.max_x.max(x + estimate_text_width(first_line));
                }
                // Track max_y for all lines in the leaf
                let mut leaf_y = y;
                for _line in lines {
                    self.max_y = self.max_y.max(leaf_y);
                    leaf_y += 30.0;
                }
            }
        }
    }
}

fn estimate_text_width(text: &str) -> f64 {
    // Rough estimate: 0.6 * font_size * char_count
    text.len() as f64 * 20.0 * 0.6
}

fn calculate_tree_height(tree: &Tree) -> f64 {
    match tree {
        Tree::Node(_, children) => {
            if children.is_empty() {
                30.0
            } else {
                // Node itself takes 30px, plus all children
                let mut total = 30.0;
                for child in children {
                    total += calculate_tree_height(child);
                }
                total
            }
        }
        Tree::Leaf(lines) => (lines.len() as f64) * 30.0,
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
