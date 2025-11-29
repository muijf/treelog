//! SVG export functionality for Tree.

use crate::tree::Tree;

impl Tree {
    /// Exports the tree as SVG tree diagram.
    ///
    /// Requires the `export-svg` feature.
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

fn svg_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
