//! Iterator API for line-by-line tree access.

use crate::config::RenderConfig;
use crate::tree::Tree;

/// Represents a single line in the rendered tree.
///
/// This struct provides information about each line, including its prefix,
/// content, depth in the tree, and whether it's the last child at its level.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// The prefix string (tree characters) for this line
    pub prefix: String,
    /// The content of this line
    pub content: String,
    /// The depth of this line in the tree (0 = root)
    pub depth: usize,
    /// Whether this is the last child at its level
    pub is_last: bool,
}

/// State for processing a leaf with multiple lines
struct LeafState {
    lines: Vec<String>,
    index: usize,
    prefix: String,
    second_line_prefix: String,
    level: Vec<usize>,
}

/// An iterator that yields lines of a rendered tree one at a time.
///
/// This allows streaming access to tree lines without materializing
/// the entire tree string in memory.
///
/// # Examples
///
/// ```
/// use treelog::{Tree, iterator::TreeLines};
///
/// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
/// let lines: Vec<_> = TreeLines::new(&tree).collect();
/// assert_eq!(lines.len(), 2);
/// ```
pub struct TreeLines<'a> {
    tree: &'a Tree,
    config: RenderConfig,
    // Stack: (child_index, parent_tree, level_info)
    stack: Vec<(usize, &'a Tree, Vec<usize>)>,
    leaf_state: Option<LeafState>,
    root_yielded: bool,
}

impl<'a> TreeLines<'a> {
    /// Creates a new iterator over the lines of a tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::{Tree, iterator::TreeLines};
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let mut lines = TreeLines::new(&tree);
    /// ```
    pub fn new(tree: &'a Tree) -> Self {
        Self::with_config(tree, &RenderConfig::default())
    }

    /// Creates a new iterator with a custom configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::{Tree, TreeStyle, RenderConfig, iterator::TreeLines};
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let config = RenderConfig::default().with_style(TreeStyle::Ascii);
    /// let mut lines = TreeLines::with_config(&tree, &config);
    /// ```
    pub fn with_config(tree: &'a Tree, config: &RenderConfig) -> Self {
        let mut stack = Vec::new();
        // Initialize stack with root's children if it's a node
        if let Tree::Node(_, children) = tree {
            if !children.is_empty() {
                stack.push((0, tree, Vec::new()));
            }
        }

        TreeLines {
            tree,
            config: config.clone(),
            stack,
            leaf_state: None,
            root_yielded: false,
        }
    }

    fn build_prefix(level: &[usize], style: &crate::style::StyleConfig) -> String {
        let mut prefix = String::new();
        let maxpos = level.len();
        for (pos, &is_last) in level.iter().enumerate() {
            let last_row = pos == maxpos - 1;
            if is_last == 1 {
                if !last_row {
                    prefix.push_str(style.get_empty());
                } else {
                    prefix.push_str(style.get_branch(true));
                }
            } else {
                if !last_row {
                    prefix.push_str(style.get_vertical());
                } else {
                    prefix.push_str(style.get_branch(false));
                }
            }
        }
        prefix
    }

    fn build_second_line_prefix(level: &[usize], style: &crate::style::StyleConfig) -> String {
        let mut prefix = String::new();
        for &is_last in level {
            if is_last == 1 {
                prefix.push_str(style.get_empty());
            } else {
                prefix.push_str(style.get_vertical());
            }
        }
        prefix
    }
}

impl<'a> Iterator for TreeLines<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        // First, yield the root node if we haven't yet
        if !self.root_yielded {
            self.root_yielded = true;
            match self.tree {
                Tree::Node(label, _) => {
                    let formatted = self.config.format_node(label);
                    return Some(Line {
                        prefix: String::new(),
                        content: formatted,
                        depth: 0,
                        is_last: true,
                    });
                }
                Tree::Leaf(lines) => {
                    // Root is a leaf - process it immediately
                    if lines.is_empty() {
                        return None;
                    }
                    let formatted = self.config.format_leaf(&lines[0]);
                    if lines.len() == 1 {
                        return Some(Line {
                            prefix: String::new(),
                            content: formatted,
                            depth: 0,
                            is_last: true,
                        });
                    }
                    // Multiple lines - set up leaf state
                    let second_prefix = Self::build_second_line_prefix(&[], &self.config.style);
                    self.leaf_state = Some(LeafState {
                        lines: lines.clone(),
                        index: 0,
                        prefix: String::new(),
                        second_line_prefix: second_prefix,
                        level: Vec::new(),
                    });
                    return self.next();
                }
            }
        }

        // Process current leaf lines if any
        if let Some(ref mut leaf_state) = self.leaf_state {
            if leaf_state.index < leaf_state.lines.len() {
                let line = leaf_state.lines[leaf_state.index].clone();
                let formatted = self.config.format_leaf(&line);
                let prefix = if leaf_state.index == 0 {
                    leaf_state.prefix.clone()
                } else {
                    leaf_state.second_line_prefix.clone()
                };
                let depth = leaf_state.level.len();
                let is_last = false;

                leaf_state.index += 1;
                let should_clear = leaf_state.index >= leaf_state.lines.len();

                if should_clear {
                    self.leaf_state = None;
                }

                return Some(Line {
                    prefix,
                    content: formatted,
                    depth,
                    is_last,
                });
            } else {
                self.leaf_state = None;
            }
        }

        // Process the stack
        while let Some((child_idx, parent, level)) = self.stack.pop() {
            match parent {
                Tree::Node(_, children) => {
                    if child_idx >= children.len() {
                        continue;
                    }

                    let child = &children[child_idx];
                    let is_last = child_idx == children.len() - 1;
                    let mut new_level = level.clone();
                    new_level.push(if is_last { 1 } else { 0 });

                    match child {
                        Tree::Node(label, grand_children) => {
                            let prefix = Self::build_prefix(&level, &self.config.style);
                            let formatted = self.config.format_node(label);
                            let depth = level.len();

                            // Push remaining siblings
                            if child_idx + 1 < children.len() {
                                self.stack.push((child_idx + 1, parent, level));
                            }
                            // Push this node's children
                            if !grand_children.is_empty() {
                                self.stack.push((0, child, new_level));
                            }

                            return Some(Line {
                                prefix,
                                content: formatted,
                                depth,
                                is_last,
                            });
                        }
                        Tree::Leaf(lines) => {
                            if lines.is_empty() {
                                continue;
                            }

                            let prefix = Self::build_prefix(&level, &self.config.style);
                            let second_prefix =
                                Self::build_second_line_prefix(&level, &self.config.style);

                            let depth = level.len();
                            if lines.len() == 1 {
                                // Single line leaf - yield immediately
                                let formatted = self.config.format_leaf(&lines[0]);
                                // Push remaining siblings
                                if child_idx + 1 < children.len() {
                                    self.stack.push((child_idx + 1, parent, level));
                                }
                                return Some(Line {
                                    prefix,
                                    content: formatted,
                                    depth,
                                    is_last,
                                });
                            } else {
                                // Multiple lines - set up leaf state
                                let level_clone = level.clone();
                                self.leaf_state = Some(LeafState {
                                    lines: lines.clone(),
                                    index: 0,
                                    prefix: prefix.clone(),
                                    second_line_prefix: second_prefix,
                                    level: level_clone,
                                });
                                // Push remaining siblings
                                if child_idx + 1 < children.len() {
                                    self.stack.push((child_idx + 1, parent, level));
                                }
                                return self.next();
                            }
                        }
                    }
                }
                Tree::Leaf(_) => unreachable!(),
            }
        }

        None
    }
}

/// Extension trait for Tree to provide iterator methods.
pub trait TreeIteratorExt {
    /// Returns an iterator over the lines of this tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::{Tree, iterator::TreeIteratorExt};
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// for line in TreeIteratorExt::lines(&tree) {
    ///     println!("{} {}", line.prefix, line.content);
    /// }
    /// ```
    fn lines(&self) -> TreeLines<'_>;

    /// Returns an iterator with a custom configuration.
    fn lines_with_config(&self, config: &RenderConfig) -> TreeLines<'_>;

    /// Collects all lines into a Vec<String>.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::{Tree, iterator::TreeIteratorExt};
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let lines: Vec<String> = tree.to_lines();
    /// ```
    fn to_lines(&self) -> Vec<String>;

    /// Collects all lines into a Vec<String> with a custom configuration.
    fn to_lines_with_config(&self, config: &RenderConfig) -> Vec<String>;
}

impl TreeIteratorExt for Tree {
    fn lines(&self) -> TreeLines<'_> {
        TreeLines::new(self)
    }

    fn lines_with_config(&self, config: &RenderConfig) -> TreeLines<'_> {
        TreeLines::with_config(self, config)
    }

    fn to_lines(&self) -> Vec<String> {
        TreeIteratorExt::lines(self)
            .map(|line| format!("{}{}", line.prefix, line.content))
            .collect()
    }

    fn to_lines_with_config(&self, config: &RenderConfig) -> Vec<String> {
        TreeIteratorExt::lines_with_config(self, config)
            .map(|line| format!("{}{}", line.prefix, line.content))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_lines_simple() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let lines: Vec<_> = TreeLines::new(&tree).collect();
        assert!(lines.len() >= 2);
    }

    #[test]
    fn test_to_lines() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let lines = tree.to_lines();
        assert!(!lines.is_empty());
    }
}

