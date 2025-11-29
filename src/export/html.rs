//! HTML export functionality for Tree.

use crate::tree::Tree;

impl Tree {
    /// Exports the tree as HTML with collapsible nodes.
    ///
    /// Requires the `export-html` feature.
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
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
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
}
