//! Tree traversal iterators for different traversal orders.

use crate::tree::Tree;

impl Tree {
    /// Returns an iterator that traverses the tree in pre-order (root, then children).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for node in tree.pre_order() {
    ///     println!("{:?}", node);
    /// }
    /// ```
    pub fn pre_order(&self) -> PreOrder<'_> {
        PreOrder::new(self)
    }

    /// Returns an iterator that traverses the tree in post-order (children, then root).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for node in tree.post_order() {
    ///     println!("{:?}", node);
    /// }
    /// ```
    pub fn post_order(&self) -> PostOrder<'_> {
        PostOrder::new(self)
    }

    /// Returns an iterator that traverses the tree in level-order (breadth-first).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for node in tree.level_order() {
    ///     println!("{:?}", node);
    /// }
    /// ```
    pub fn level_order(&self) -> LevelOrder<'_> {
        LevelOrder::new(self)
    }

    /// Returns an iterator over all nodes (excluding leaves).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for node in tree.nodes() {
    ///     println!("Node: {}", node.label().unwrap());
    /// }
    /// ```
    pub fn nodes(&self) -> Nodes<'_> {
        Nodes::new(self)
    }

    /// Returns an iterator over all leaves (excluding nodes).
    ///
    /// Requires the `traversal` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![
    ///     Tree::Leaf(vec!["item".to_string()])
    /// ]);
    /// for leaf in tree.leaves() {
    ///     println!("Leaf: {:?}", leaf.lines());
    /// }
    /// ```
    pub fn leaves(&self) -> Leaves<'_> {
        Leaves::new(self)
    }
}

/// An iterator that traverses a tree in pre-order (root, then children).
///
/// # Examples
///
/// ```
/// use treelog::{Tree, ops::traversal::PreOrder};
///
/// let tree = Tree::Node("root".to_string(), vec![
///     Tree::Leaf(vec!["item".to_string()])
/// ]);
/// let nodes: Vec<_> = PreOrder::new(&tree).collect();
/// ```
pub struct PreOrder<'a> {
    stack: Vec<&'a Tree>,
}

impl<'a> PreOrder<'a> {
    /// Creates a new pre-order iterator.
    pub fn new(tree: &'a Tree) -> Self {
        PreOrder { stack: vec![tree] }
    }
}

impl<'a> Iterator for PreOrder<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().inspect(|tree| {
            if let Tree::Node(_, children) = tree {
                // Push children in reverse order so we process them left-to-right
                for child in children.iter().rev() {
                    self.stack.push(child);
                }
            }
        })
    }
}

/// An iterator that traverses a tree in post-order (children, then root).
///
/// # Examples
///
/// ```
/// use treelog::{Tree, ops::traversal::PostOrder};
///
/// let tree = Tree::Node("root".to_string(), vec![
///     Tree::Leaf(vec!["item".to_string()])
/// ]);
/// let nodes: Vec<_> = PostOrder::new(&tree).collect();
/// ```
pub struct PostOrder<'a> {
    stack: Vec<(bool, &'a Tree)>,
}

impl<'a> PostOrder<'a> {
    /// Creates a new post-order iterator.
    pub fn new(tree: &'a Tree) -> Self {
        PostOrder {
            stack: vec![(false, tree)],
        }
    }
}

impl<'a> Iterator for PostOrder<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((visited, tree)) = self.stack.pop() {
            if visited {
                return Some(tree);
            }

            // Mark as visited and push back
            self.stack.push((true, tree));

            // Push children in reverse order
            if let Tree::Node(_, children) = tree {
                for child in children.iter().rev() {
                    self.stack.push((false, child));
                }
            }
        }
        None
    }
}

/// An iterator that traverses a tree in level-order (breadth-first).
///
/// # Examples
///
/// ```
/// use treelog::{Tree, ops::traversal::LevelOrder};
///
/// let tree = Tree::Node("root".to_string(), vec![
///     Tree::Leaf(vec!["item".to_string()])
/// ]);
/// let nodes: Vec<_> = LevelOrder::new(&tree).collect();
/// ```
pub struct LevelOrder<'a> {
    queue: std::collections::VecDeque<&'a Tree>,
}

impl<'a> LevelOrder<'a> {
    /// Creates a new level-order iterator.
    pub fn new(tree: &'a Tree) -> Self {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(tree);
        LevelOrder { queue }
    }
}

impl<'a> Iterator for LevelOrder<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().inspect(|tree| {
            if let Tree::Node(_, children) = tree {
                for child in children {
                    self.queue.push_back(child);
                }
            }
        })
    }
}

/// An iterator that yields only nodes (not leaves).
///
/// # Examples
///
/// ```
/// use treelog::{Tree, ops::traversal::Nodes};
///
/// let tree = Tree::Node("root".to_string(), vec![
///     Tree::Leaf(vec!["item".to_string()])
/// ]);
/// let nodes: Vec<_> = Nodes::new(&tree).collect();
/// ```
pub struct Nodes<'a> {
    inner: PreOrder<'a>,
}

impl<'a> Nodes<'a> {
    /// Creates a new nodes iterator.
    pub fn new(tree: &'a Tree) -> Self {
        Nodes {
            inner: PreOrder::new(tree),
        }
    }
}

impl<'a> Iterator for Nodes<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find(|tree| tree.is_node())
    }
}

/// An iterator that yields only leaves (not nodes).
///
/// # Examples
///
/// ```
/// use treelog::{Tree, ops::traversal::Leaves};
///
/// let tree = Tree::Node("root".to_string(), vec![
///     Tree::Leaf(vec!["item".to_string()])
/// ]);
/// let leaves: Vec<_> = Leaves::new(&tree).collect();
/// ```
pub struct Leaves<'a> {
    inner: PreOrder<'a>,
}

impl<'a> Leaves<'a> {
    /// Creates a new leaves iterator.
    pub fn new(tree: &'a Tree) -> Self {
        Leaves {
            inner: PreOrder::new(tree),
        }
    }
}

impl<'a> Iterator for Leaves<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find(|tree| tree.is_leaf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_order() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node("a".to_string(), vec![Tree::Leaf(vec!["a1".to_string()])]),
                Tree::Leaf(vec!["b".to_string()]),
            ],
        );
        let nodes: Vec<_> = PreOrder::new(&tree).collect();
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].label(), Some("root"));
    }

    #[test]
    fn test_post_order() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node("a".to_string(), vec![Tree::Leaf(vec!["a1".to_string()])]),
                Tree::Leaf(vec!["b".to_string()]),
            ],
        );
        let nodes: Vec<_> = PostOrder::new(&tree).collect();
        assert_eq!(nodes.len(), 4);
        // Last should be root
        assert_eq!(nodes[3].label(), Some("root"));
    }

    #[test]
    fn test_level_order() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node("a".to_string(), vec![Tree::Leaf(vec!["a1".to_string()])]),
                Tree::Leaf(vec!["b".to_string()]),
            ],
        );
        let nodes: Vec<_> = LevelOrder::new(&tree).collect();
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].label(), Some("root"));
    }

    #[test]
    fn test_nodes() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node("a".to_string(), vec![Tree::Leaf(vec!["a1".to_string()])]),
                Tree::Leaf(vec!["b".to_string()]),
            ],
        );
        let nodes: Vec<_> = Nodes::new(&tree).collect();
        assert_eq!(nodes.len(), 2);
        assert!(nodes.iter().all(|n| n.is_node()));
    }

    #[test]
    fn test_leaves() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Node("a".to_string(), vec![Tree::Leaf(vec!["a1".to_string()])]),
                Tree::Leaf(vec!["b".to_string()]),
            ],
        );
        let leaves: Vec<_> = Leaves::new(&tree).collect();
        assert_eq!(leaves.len(), 2);
        assert!(leaves.iter().all(|l| l.is_leaf()));
    }
}
