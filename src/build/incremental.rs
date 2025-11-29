//! Incremental tree construction for dynamic tree building.
//!
//! This module provides an `IncrementalTree` that allows you to build a `Tree` enum
//! structure incrementally. Items can be added one at a time, and the complete
//! `Tree` structure can be retrieved at any point.

//! # Examples
//!
//! ```no_run
//! use treelog::{build::incremental::IncrementalTree, Tree};
//!
//! let mut tree = IncrementalTree::new();
//! let root_id = tree.add_node("root", None);
//! let child_id = tree.add_leaf("child", Some(root_id));
//!
//! // Get the complete Tree structure
//! let tree_enum = tree.build_tree().unwrap();
//!
//! // Get prefix for an item (uses Tree iterator internally)
//! let prefix = tree.get_prefix(child_id);
//! ```

use std::collections::HashMap;

use crate::render::level::LevelPath;
use crate::render::prefix::compute_prefix;
use crate::render::style::StyleConfig;
use crate::tree::Tree;

/// A dynamic tree that builds a `Tree` enum structure incrementally.
///
/// Unlike [`TreeLines`](crate::iterator::TreeLines) which iterates over a static
/// `Tree` enum, `IncrementalTree` allows you to build a tree incrementally by adding
/// nodes and leaves one at a time. It maintains the structure internally and can
/// always produce a complete `Tree` enum structure.
///
/// This type can be used directly for incremental tree-building scenarios, such as
/// managing hierarchical progress bars with the `indicatif` crate.
///
/// # Examples
///
/// ```no_run
/// use treelog::build::IncrementalTree;
///
/// let mut tree = IncrementalTree::new();
/// let root_id = tree.add_node("root", None);
/// let child_id = tree.add_leaf("child", Some(root_id));
///
/// // Build the complete Tree structure
/// let tree_enum = tree.build_tree().unwrap();
/// ```
pub struct IncrementalTree {
    /// Maps item ID to Tree node/leaf
    trees: HashMap<usize, Tree>,
    /// Maps parent ID to list of child IDs
    parent_to_children: HashMap<usize, Vec<usize>>,
    /// Maps child ID to parent ID (for quick lookups)
    child_to_parent: HashMap<usize, Option<usize>>,
    /// Next available ID counter
    next_id: usize,
    /// Root node IDs (can be multiple if no single root)
    root_ids: Vec<usize>,
    /// Style configuration for prefix computation
    style: StyleConfig,
}

impl IncrementalTree {
    /// Creates a new `IncrementalTree` with default style configuration.
    pub fn new() -> Self {
        Self::with_style(StyleConfig::default())
    }

    /// Creates a new `IncrementalTree` with a custom style configuration.
    pub fn with_style(style: StyleConfig) -> Self {
        Self {
            trees: HashMap::new(),
            parent_to_children: HashMap::new(),
            child_to_parent: HashMap::new(),
            next_id: 0,
            root_ids: Vec::new(),
            style,
        }
    }

    /// Helper method to link a child to its parent or mark it as root.
    fn link_to_parent(&mut self, id: usize, parent_id: Option<usize>) {
        if let Some(parent_id) = parent_id {
            // Verify parent exists and is a node
            if let Some(Tree::Node(_, _)) = self.trees.get(&parent_id) {
                self.parent_to_children
                    .entry(parent_id)
                    .or_default()
                    .push(id);
                self.child_to_parent.insert(id, Some(parent_id));
            } else {
                // Parent doesn't exist or isn't a node - treat as root
                // This can happen if items are added out of order
                self.root_ids.push(id);
                self.child_to_parent.insert(id, None);
            }
        } else {
            // This is a root item
            self.root_ids.push(id);
            self.child_to_parent.insert(id, None);
        }
    }

    /// Adds a node to the tree with an optional parent.
    ///
    /// # Arguments
    ///
    /// * `label` - The label for the node
    /// * `parent_id` - The ID of the parent node, or `None` for a root node
    ///
    /// # Returns
    ///
    /// The unique ID assigned to this node, which can be used as a parent for child items.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::IncrementalTree;
    ///
    /// let mut tree = IncrementalTree::new();
    /// let root_id = tree.add_node("root", None);
    /// let child_id = tree.add_node("child", Some(root_id));
    /// ```
    pub fn add_node(&mut self, label: impl Into<String>, parent_id: Option<usize>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let node = Tree::new_node(label);
        self.trees.insert(id, node);
        self.link_to_parent(id, parent_id);

        id
    }

    /// Adds a single-line leaf to the tree with an optional parent.
    ///
    /// # Arguments
    ///
    /// * `line` - The line of text for the leaf
    /// * `parent_id` - The ID of the parent node, or `None` for a root leaf
    ///
    /// # Returns
    ///
    /// The unique ID assigned to this leaf.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::IncrementalTree;
    ///
    /// let mut tree = IncrementalTree::new();
    /// let root_id = tree.add_node("root", None);
    /// let leaf_id = tree.add_leaf("item", Some(root_id));
    /// ```
    #[allow(dead_code)]
    pub fn add_leaf(&mut self, line: impl Into<String>, parent_id: Option<usize>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let leaf = Tree::new_leaf(line);
        self.trees.insert(id, leaf);
        self.link_to_parent(id, parent_id);

        id
    }

    /// Adds a multi-line leaf to the tree with an optional parent.
    ///
    /// # Arguments
    ///
    /// * `lines` - The lines of text for the leaf
    /// * `parent_id` - The ID of the parent node, or `None` for a root leaf
    ///
    /// # Returns
    ///
    /// The unique ID assigned to this leaf.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::IncrementalTree;
    ///
    /// let mut tree = IncrementalTree::new();
    /// let root_id = tree.add_node("root", None);
    /// let leaf_id = tree.add_leaf_lines(vec!["line1", "line2"], Some(root_id));
    /// ```
    #[allow(dead_code)]
    pub fn add_leaf_lines(
        &mut self,
        lines: Vec<impl Into<String>>,
        parent_id: Option<usize>,
    ) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let leaf = Tree::new_leaf_lines(lines);
        self.trees.insert(id, leaf);
        self.link_to_parent(id, parent_id);

        id
    }

    /// Builds and returns the complete `Tree` enum structure.
    ///
    /// If there are multiple root nodes, they are wrapped in a synthetic root node.
    /// If the tree is empty, returns `None`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::IncrementalTree;
    ///
    /// let mut tree = IncrementalTree::new();
    /// let root_id = tree.add_node("root", None);
    /// let child_id = tree.add_leaf("child", Some(root_id));
    ///
    /// let tree_enum = tree.build_tree().unwrap();
    /// ```
    #[allow(dead_code)]
    pub fn build_tree(&self) -> Option<Tree> {
        if self.trees.is_empty() {
            return None;
        }

        if self.root_ids.is_empty() {
            // This shouldn't happen if the tree is not empty, but handle it gracefully
            return None;
        }

        if self.root_ids.len() == 1 {
            // Single root - build from it
            self.build_subtree(self.root_ids[0])
        } else {
            // Multiple roots - wrap in synthetic root
            let children: Vec<Tree> = self
                .root_ids
                .iter()
                .filter_map(|&id| self.build_subtree(id))
                .collect();
            Some(Tree::Node(String::new(), children))
        }
    }

    /// Calculates the position of an existing item in the tree order.
    ///
    /// This can be used to determine where to insert items in an ordered display,
    /// such as when managing hierarchical progress bars with the `indicatif` crate.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the item to find the position for
    ///
    /// # Returns
    ///
    /// The position of the item in the ordered list (0-based).
    ///
    /// # Panics
    ///
    /// Panics if the item ID does not exist in the tree.
    pub fn calculate_insert_position_for_existing(&self, id: usize) -> usize {
        let mut pos = 0;
        let mut stack: Vec<usize> = self.root_ids.iter().rev().copied().collect();

        while let Some(current_id) = stack.pop() {
            if current_id == id {
                return pos;
            }
            pos += 1;
            if let Some(children) = self.parent_to_children.get(&current_id) {
                for &child in children.iter().rev() {
                    stack.push(child);
                }
            }
        }

        panic!("id {} must exist in tree", id);
    }

    /// Builds a subtree starting from the given ID.
    fn build_subtree(&self, id: usize) -> Option<Tree> {
        let tree = self.trees.get(&id)?.clone();

        match tree {
            Tree::Node(label, _) => {
                let children: Vec<Tree> = self
                    .parent_to_children
                    .get(&id)
                    .map(|child_ids| {
                        child_ids
                            .iter()
                            .filter_map(|&child_id| self.build_subtree(child_id))
                            .collect()
                    })
                    .unwrap_or_default();
                Some(Tree::Node(label, children))
            }
            Tree::Leaf(_) => Some(tree),
        }
    }

    /// Gets a reference to a specific Tree node/leaf by ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::IncrementalTree;
    ///
    /// let mut tree = IncrementalTree::new();
    /// let root_id = tree.add_node("root", None);
    ///
    /// let root_tree = tree.get_tree(root_id);
    /// ```
    #[allow(dead_code)]
    pub fn get_tree(&self, id: usize) -> Option<&Tree> {
        self.trees.get(&id)
    }

    /// Gets the tree prefix for a specific item ID.
    ///
    /// This computes the prefix by building a LevelPath from the parent chain
    /// and using the prefix computation logic, ensuring consistency with the
    /// main Tree rendering logic.
    ///
    /// Returns `None` if the item doesn't exist or is a root item (no prefix).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::IncrementalTree;
    ///
    /// let mut tree = IncrementalTree::new();
    /// let root_id = tree.add_node("root", None);
    /// let child_id = tree.add_leaf("child", Some(root_id));
    ///
    /// let prefix = tree.get_prefix(child_id);
    /// ```
    pub fn get_prefix(&self, id: usize) -> Option<String> {
        // Check if this is a root item
        if self.child_to_parent.get(&id) == Some(&None) {
            return None;
        }

        // Check if item exists
        if !self.trees.contains_key(&id) {
            return None;
        }

        // Build LevelPath by walking up the ancestor chain
        let level_path = LevelPath::from_parent_chain(
            id,
            |idx| self.child_to_parent.get(&idx).and_then(|parent| *parent),
            |idx| {
                // Determine if this item is the last child of its parent
                if let Some(Some(parent_id)) = self.child_to_parent.get(&idx)
                    && let Some(children) = self.parent_to_children.get(parent_id)
                    && !children.is_empty()
                {
                    // The last child in the children list is the last child
                    return children.last() == Some(&idx);
                }
                false
            },
        );

        Some(compute_prefix(&level_path, &self.style))
    }

    /// Returns the number of items in the tree.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::incremental::IncrementalTree;
    ///
    /// let mut tree = IncrementalTree::new();
    /// let root_id = tree.add_node("root", None);
    /// let child_id = tree.add_leaf("child", Some(root_id));
    /// assert_eq!(tree.len(), 2);
    /// ```
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.trees.len()
    }

    /// Returns `true` if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::incremental::IncrementalTree;
    ///
    /// let tree = IncrementalTree::new();
    /// assert!(tree.is_empty());
    ///
    /// let mut tree = IncrementalTree::new();
    /// tree.add_node("root", None);
    /// assert!(!tree.is_empty());
    /// ```
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }

    /// Returns the style configuration.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::{build::incremental::IncrementalTree, StyleConfig, TreeStyle};
    ///
    /// let tree = IncrementalTree::new();
    /// let style = tree.style();
    /// assert_eq!(style.branch, "├─ ");
    ///
    /// let custom_style = StyleConfig::from(TreeStyle::Ascii);
    /// let tree = IncrementalTree::with_style(custom_style);
    /// assert_eq!(tree.style().branch, "+- ");
    /// ```
    #[allow(dead_code)]
    pub fn style(&self) -> &StyleConfig {
        &self.style
    }

    /// Calculates the position where an item with the given parent would be inserted.
    ///
    /// This is useful when you need to know the insert position before actually
    /// inserting the item (e.g., for inserting into an external collection).
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The ID of the parent item, or `None` for a root item
    ///
    /// # Returns
    ///
    /// The position where the item would be inserted in the ordered list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::build::incremental::IncrementalTree;
    ///
    /// let mut tree = IncrementalTree::new();
    /// let root_id = tree.add_node("root", None);
    /// let child_id = tree.add_leaf("child", Some(root_id));
    ///
    /// // Calculate where a new sibling would be inserted
    /// let pos = tree.calculate_insert_position(Some(root_id));
    /// ```
    #[allow(dead_code)]
    pub fn calculate_insert_position(&self, parent_id: Option<usize>) -> usize {
        if let Some(parent_id) = parent_id {
            // Count all items up to and including the last descendant of the parent
            let mut count = 0;
            let mut stack = vec![parent_id];

            while let Some(id) = stack.pop() {
                count += 1;
                if let Some(children) = self.parent_to_children.get(&id) {
                    for &child_id in children.iter().rev() {
                        stack.push(child_id);
                    }
                }
            }

            count
        } else {
            // Root item - count all root items
            self.root_ids.len()
        }
    }
}

impl Default for IncrementalTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = IncrementalTree::new();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_add_root_node() {
        let mut tree = IncrementalTree::new();
        let id = tree.add_node("root", None);
        assert_eq!(tree.len(), 1);
        assert!(tree.get_tree(id).is_some());
        assert!(tree.get_tree(id).unwrap().is_node());
    }

    #[test]
    fn test_add_child_leaf() {
        let mut tree = IncrementalTree::new();
        let root_id = tree.add_node("root", None);
        let _child_id = tree.add_leaf("child", Some(root_id));
        assert_eq!(tree.len(), 2);
        assert!(tree.get_tree(_child_id).is_some());
        assert!(tree.get_tree(_child_id).unwrap().is_leaf());
    }

    #[test]
    fn test_build_tree() {
        let mut tree = IncrementalTree::new();
        let root_id = tree.add_node("root", None);
        let _child_id = tree.add_leaf("child", Some(root_id));

        let tree_enum = tree.build_tree().unwrap();
        assert!(tree_enum.is_node());
        if let Tree::Node(_, children) = tree_enum {
            assert_eq!(children.len(), 1);
            assert!(children[0].is_leaf());
        }
    }

    #[test]
    fn test_multiple_children() {
        let mut tree = IncrementalTree::new();
        let root_id = tree.add_node("root", None);
        let _child1_id = tree.add_leaf("child1", Some(root_id));
        let _child2_id = tree.add_leaf("child2", Some(root_id));
        assert_eq!(tree.len(), 3);

        let tree_enum = tree.build_tree().unwrap();
        if let Tree::Node(_, children) = tree_enum {
            assert_eq!(children.len(), 2);
        }
    }

    #[test]
    fn test_multiple_roots() {
        let mut tree = IncrementalTree::new();
        let _root1_id = tree.add_node("root1", None);
        let _root2_id = tree.add_node("root2", None);
        assert_eq!(tree.len(), 2);

        let tree_enum = tree.build_tree().unwrap();
        // Should be wrapped in a synthetic root
        if let Tree::Node(_, children) = tree_enum {
            assert_eq!(children.len(), 2);
        }
    }

    #[test]
    fn test_empty_tree() {
        let tree = IncrementalTree::new();
        assert!(tree.build_tree().is_none());
    }

    #[test]
    fn test_add_leaf_lines() {
        let mut tree = IncrementalTree::new();
        let root_id = tree.add_node("root", None);
        let leaf_id = tree.add_leaf_lines(vec!["line1", "line2"], Some(root_id));

        let leaf = tree.get_tree(leaf_id).unwrap();
        assert!(leaf.is_leaf());
        if let Tree::Leaf(lines) = leaf {
            assert_eq!(lines.len(), 2);
        }
    }

    #[test]
    fn test_get_prefix_for_child() {
        let mut tree = IncrementalTree::new();
        let root_id = tree.add_node("root", None);
        let child_id = tree.add_leaf("child", Some(root_id));

        // Child should have a prefix (not None)
        let prefix = tree.get_prefix(child_id);
        assert!(prefix.is_some());
        assert!(!prefix.unwrap().is_empty());
    }

    #[test]
    fn test_node_as_parent_of_node() {
        // Test that nodes can be parents of other nodes (for indicatif use case)
        let mut tree = IncrementalTree::new();
        let root_id = tree.add_node("root", None);
        let child_node_id = tree.add_node("child", Some(root_id));
        let grandchild_id = tree.add_leaf("grandchild", Some(child_node_id));

        // Both child_node and grandchild should have prefixes
        let child_prefix = tree.get_prefix(child_node_id);
        let grandchild_prefix = tree.get_prefix(grandchild_id);

        assert!(child_prefix.is_some());
        assert!(grandchild_prefix.is_some());
        // Grandchild prefix should be longer (deeper in tree)
        assert!(grandchild_prefix.unwrap().len() > child_prefix.unwrap().len());
    }

    #[test]
    fn test_multiple_siblings_last_child() {
        // Test that only the last sibling is marked as last child
        let mut tree = IncrementalTree::new();
        let root_id = tree.add_node("root", None);
        let child1_id = tree.add_leaf("child1", Some(root_id));
        let child2_id = tree.add_leaf("child2", Some(root_id));
        let child3_id = tree.add_leaf("child3", Some(root_id));

        let prefix1 = tree.get_prefix(child1_id).unwrap();
        let prefix2 = tree.get_prefix(child2_id).unwrap();
        let prefix3 = tree.get_prefix(child3_id).unwrap();

        // child1 and child2 should not be last (use ├─)
        assert!(prefix1.contains("├"));
        assert!(prefix2.contains("├"));
        // child3 should be last (use └─)
        assert!(prefix3.contains("└"));
        assert!(!prefix3.contains("├"));
    }
}
