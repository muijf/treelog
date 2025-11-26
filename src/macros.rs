//! Macro-based DSL for constructing trees.
//!
//! This module provides the `tree!` macro which allows you to create trees
//! using a clean, declarative syntax.
//!
//! # Examples
//!
//! ```
//! use treelog::tree;
//!
//! let tree = tree! {
//!     root {
//!         "item1",
//!         "item2",
//!         sub {
//!             "subitem1",
//!             "subitem2"
//!         }
//!     }
//! };
//! ```
//!
//! The macro supports:
//! - **Nodes**: `identifier { ... }` or `"string" { ... }`
//! - **Leaves**: `"string"` or bare identifiers (treated as strings)
//! - **Nested structures**: Arbitrary nesting depth
//! - **Comma-separated**: Children separated by commas (trailing comma optional)

/// Creates a tree using a DSL-like syntax.
///
/// This macro provides a convenient way to construct trees with a clean,
/// declarative syntax that mirrors the tree structure itself.
///
/// # Syntax
///
/// - **Nodes**: `name { children... }` or `"name" { children... }`
/// - **Leaves**: `"text"` or bare identifiers (converted to strings)
/// - **Children**: Comma-separated list (trailing comma optional)
///
/// # Examples
///
/// Simple tree with leaves:
/// ```
/// # #[cfg(feature = "macro")]
/// # {
/// use treelog::tree;
///
/// let tree = tree! {
///     root {
///         "item1",
///         "item2"
///     }
/// };
/// # }
/// ```
///
/// Nested nodes:
/// ```
/// # #[cfg(feature = "macro")]
/// # {
/// use treelog::tree;
///
/// let tree = tree! {
///     root {
///         "item1",
///         sub {
///             "subitem1",
///             "subitem2"
///         }
///     }
/// };
/// # }
/// ```
///
/// Using string literals for node names:
/// ```
/// # #[cfg(feature = "macro")]
/// # {
/// use treelog::tree;
///
/// let tree = tree! {
///     "root" {
///         "item1",
///         "sub node" {
///             "subitem"
///         }
///     }
/// };
/// # }
/// ```
///
/// Bare identifiers as leaves:
/// ```
/// # #[cfg(feature = "macro")]
/// # {
/// use treelog::tree;
///
/// let tree = tree! {
///     root {
///         item1,
///         item2,
///         sub {
///             subitem
///         }
///     }
/// };
/// # }
/// ```
#[macro_export]
macro_rules! tree {
    // Entry point: single node with identifier
    ($name:ident { $($children:tt)* }) => {
        $crate::Tree::Node(
            stringify!($name).to_string(),
            $crate::__tree_parse_children!([$($children)*] [])
        )
    };

    // Entry point: single node with string
    ($name:literal { $($children:tt)* }) => {
        $crate::Tree::Node(
            $name.to_string(),
            $crate::__tree_parse_children!([$($children)*] [])
        )
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __tree_parse_children {
    // Node with identifier followed by comma
    ([$i:ident { $($s:tt)* } , $($rest:tt)*] [$($acc:expr,)*]) => {
        $crate::__tree_parse_children!([$($rest)*] [$($acc,)* $crate::tree!($i { $($s)* }),])
    };

    // Node with string followed by comma
    ([$l:literal { $($s:tt)* } , $($rest:tt)*] [$($acc:expr,)*]) => {
        $crate::__tree_parse_children!([$($rest)*] [$($acc,)* $crate::tree!($l { $($s)* }),])
    };

    // Leaf with string followed by comma
    ([$l:literal , $($rest:tt)*] [$($acc:expr,)*]) => {
        $crate::__tree_parse_children!([$($rest)*] [$($acc,)* $crate::Tree::Leaf(vec![$l.to_string()]),])
    };

    // Leaf with identifier followed by comma
    ([$i:ident , $($rest:tt)*] [$($acc:expr,)*]) => {
        $crate::__tree_parse_children!([$($rest)*] [$($acc,)* $crate::Tree::Leaf(vec![stringify!($i).to_string()]),])
    };

    // Last: node with identifier
    ([$i:ident { $($s:tt)* }] [$($acc:expr,)*]) => {
        vec![$($acc,)* $crate::tree!($i { $($s)* })]
    };

    // Last: node with string
    ([$l:literal { $($s:tt)* }] [$($acc:expr,)*]) => {
        vec![$($acc,)* $crate::tree!($l { $($s)* })]
    };

    // Last: leaf with string
    ([$l:literal] [$($acc:expr,)*]) => {
        vec![$($acc,)* $crate::Tree::Leaf(vec![$l.to_string()])]
    };

    // Last: leaf with identifier
    ([$i:ident] [$($acc:expr,)*]) => {
        vec![$($acc,)* $crate::Tree::Leaf(vec![stringify!($i).to_string()])]
    };

    // Base case: empty input (with or without trailing comma), return accumulated
    ([] [$($acc:expr,)*]) => {
        vec![$($acc,)*]
    };

    // Handle trailing comma before base case
    ([,] [$($acc:expr,)*]) => {
        vec![$($acc,)*]
    };
}

#[cfg(test)]
mod tests {
    use crate::tree::Tree;

    #[test]
    fn test_simple_tree() {
        let tree = tree! {
            root {
                "item1",
                "item2"
            }
        };

        assert!(tree.is_node());
        assert_eq!(tree.label(), Some("root"));
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 2);
            assert!(children[0].is_leaf());
            assert!(children[1].is_leaf());
        }
    }

    #[test]
    fn test_nested_tree() {
        let tree = tree! {
            root {
                "item1",
                sub {
                    "subitem1",
                    "subitem2"
                }
            }
        };

        assert!(tree.is_node());
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 2);
            assert!(children[0].is_leaf());
            assert!(children[1].is_node());
            if let Tree::Node(_, subchildren) = &children[1] {
                assert_eq!(subchildren.len(), 2);
            }
        }
    }

    #[test]
    fn test_string_node_name() {
        let tree = tree! {
            "root node" {
                "item1"
            }
        };

        assert!(tree.is_node());
        assert_eq!(tree.label(), Some("root node"));
    }

    #[test]
    fn test_identifier_leaves() {
        let tree = tree! {
            root {
                item1,
                item2
            }
        };

        assert!(tree.is_node());
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 2);
            if let Tree::Leaf(lines) = &children[0] {
                assert_eq!(lines[0], "item1");
            }
            if let Tree::Leaf(lines) = &children[1] {
                assert_eq!(lines[0], "item2");
            }
        }
    }

    #[test]
    fn test_trailing_comma() {
        let tree = tree! {
            root {
                "item1",
                "item2",
            }
        };

        assert!(tree.is_node());
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 2);
        }
    }

    #[test]
    fn test_empty_node() {
        let tree = tree! {
            root {}
        };

        assert!(tree.is_node());
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 0);
        }
    }

    #[test]
    fn test_deeply_nested() {
        let tree = tree! {
            level1 {
                level2 {
                    level3 {
                        "leaf"
                    }
                }
            }
        };

        assert!(tree.is_node());
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 1);
            if let Tree::Node(_, children) = &children[0] {
                assert_eq!(children.len(), 1);
                if let Tree::Node(_, children) = &children[0] {
                    assert_eq!(children.len(), 1);
                    assert!(children[0].is_leaf());
                }
            }
        }
    }

    #[test]
    fn test_mixed_syntax() {
        let tree = tree! {
            "root" {
                item1,
                "item2",
                sub {
                    "subitem1",
                    subitem2
                }
            }
        };

        assert!(tree.is_node());
        if let Tree::Node(_, children) = &tree {
            assert_eq!(children.len(), 3);
        }
    }
}
