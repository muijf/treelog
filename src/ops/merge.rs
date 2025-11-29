//! Tree merging operations.

use crate::tree::Tree;

#[cfg(feature = "clap")]
use clap::ValueEnum;

/// Strategy for merging two trees.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum MergeStrategy {
    /// Replace nodes/leaves in the first tree with those from the second tree
    Replace,
    /// Append children from the second tree to nodes in the first tree
    Append,
    /// Merge nodes with matching labels, append children otherwise
    #[cfg_attr(feature = "clap", value(name = "merge-by-label"))]
    MergeByLabel,
}

impl Tree {
    /// Merges this tree with another tree using the specified strategy.
    ///
    /// Returns a new merged tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::{Tree, ops::merge::MergeStrategy};
    ///
    /// let tree1 = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["a".to_string()])
    /// ]);
    /// let tree2 = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["b".to_string()])
    /// ]);
    /// let merged = tree1.merge(tree2, MergeStrategy::Append);
    /// ```
    pub fn merge(&self, other: Tree, strategy: crate::ops::merge::MergeStrategy) -> Tree {
        use crate::ops::merge::MergeStrategy;
        match strategy {
            MergeStrategy::Replace => self.merge_replace(&other),
            MergeStrategy::Append => self.merge_append(&other),
            MergeStrategy::MergeByLabel => self.merge_by_label(&other),
        }
    }

    fn merge_replace(&self, other: &Tree) -> Tree {
        // Simply return a clone of the other tree
        other.clone()
    }

    fn merge_append(&self, other: &Tree) -> Tree {
        match (self, other) {
            (Tree::Node(label1, children1), Tree::Node(_label2, children2)) => {
                let mut merged_children = children1.clone();
                merged_children.extend(children2.iter().cloned());
                Tree::Node(label1.clone(), merged_children)
            }
            (Tree::Leaf(_), Tree::Leaf(lines2)) => Tree::Leaf(lines2.clone()),
            (Tree::Node(label, children), Tree::Leaf(_)) => {
                Tree::Node(label.clone(), children.clone())
            }
            (Tree::Leaf(_), Tree::Node(label, children)) => {
                Tree::Node(label.clone(), children.clone())
            }
        }
    }

    fn merge_by_label(&self, other: &Tree) -> Tree {
        match (self, other) {
            (Tree::Node(label1, children1), Tree::Node(_label2, children2)) => {
                if label1 == _label2 {
                    // Merge children by matching labels
                    let mut merged_children = Vec::new();
                    let mut used_indices = std::collections::HashSet::new();

                    // First, add all children from self
                    for child1 in children1.iter() {
                        if let Tree::Node(child_label, _) = child1 {
                            // Try to find matching child in other
                            if let Some((index, child2)) =
                                children2.iter().enumerate().find(|(i, c)| {
                                    !used_indices.contains(i)
                                        && matches!(c, Tree::Node(l, _) if l == child_label)
                                })
                            {
                                used_indices.insert(index);
                                merged_children.push(child1.merge_by_label(child2));
                            } else {
                                merged_children.push(child1.clone());
                            }
                        } else {
                            merged_children.push(child1.clone());
                        }
                    }

                    // Add remaining children from other
                    for (index, child2) in children2.iter().enumerate() {
                        if !used_indices.contains(&index) {
                            merged_children.push(child2.clone());
                        }
                    }

                    Tree::Node(label1.clone(), merged_children)
                } else {
                    // Different labels, append
                    self.merge_append(other)
                }
            }
            (Tree::Leaf(_), Tree::Leaf(lines2)) => Tree::Leaf(lines2.clone()),
            _ => self.merge_append(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_replace() {
        let tree1 = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["a".to_string()])]);
        let tree2 = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["b".to_string()])]);
        let merged = tree1.merge(tree2, MergeStrategy::Replace);
        if let Tree::Node(_, children) = &merged
            && let Tree::Leaf(lines) = &children[0]
        {
            assert_eq!(lines[0], "b");
        }
    }

    #[test]
    fn test_merge_append() {
        let tree1 = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["a".to_string()])]);
        let tree2 = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["b".to_string()])]);
        let merged = tree1.merge(tree2, MergeStrategy::Append);
        if let Tree::Node(_, children) = &merged {
            assert_eq!(children.len(), 2);
        }
    }

    #[test]
    fn test_merge_by_label() {
        let tree1 = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![Tree::Leaf(vec!["a".to_string()])],
            )],
        );
        let tree2 = Tree::Node(
            "root".to_string(),
            vec![Tree::Node(
                "child".to_string(),
                vec![Tree::Leaf(vec!["b".to_string()])],
            )],
        );
        let merged = tree1.merge(tree2, MergeStrategy::MergeByLabel);
        if let Tree::Node(_, children) = &merged {
            assert_eq!(children.len(), 1);
        }
    }
}
