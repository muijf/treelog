//! Internal utilities for tree rendering.

/// Calculates the estimated capacity needed for rendering a tree.
///
/// This is a heuristic that helps pre-allocate string capacity
/// to reduce allocations during rendering.
pub(crate) fn estimate_capacity(tree: &crate::tree::Tree, avg_line_len: usize) -> usize {
    fn count_nodes_and_lines(tree: &crate::tree::Tree) -> (usize, usize) {
        match tree {
            crate::tree::Tree::Node(_, children) => {
                let mut nodes = 1;
                let mut lines = 0;
                for child in children {
                    let (n, l) = count_nodes_and_lines(child);
                    nodes += n;
                    lines += l;
                }
                (nodes, lines)
            }
            crate::tree::Tree::Leaf(leaf_lines) => (0, leaf_lines.len()),
        }
    }

    let (nodes, lines) = count_nodes_and_lines(tree);
    // Estimate: each node/line needs prefix (~10 chars) + content + newline
    (nodes + lines) * (10 + avg_line_len + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Tree;

    #[test]
    fn test_estimate_capacity() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["line1".to_string()]),
                Tree::Leaf(vec!["line2".to_string()]),
            ],
        );
        let capacity = estimate_capacity(&tree, 10);
        assert!(capacity > 0);
    }
}
