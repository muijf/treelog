//! Tree operations module.
//!
//! This module contains various operations for manipulating, querying, and analyzing trees.

// Declare sub-modules
#[cfg(any(feature = "compare", doc))]
pub mod compare;
#[cfg(any(feature = "merge", doc))]
pub mod merge;
#[cfg(any(feature = "path", doc))]
pub mod path;
#[cfg(any(feature = "search", doc))]
pub mod search;
#[cfg(any(feature = "sort", doc))]
pub mod sort;
#[cfg(any(feature = "stats", doc))]
pub mod stats;
#[cfg(any(feature = "transform", doc))]
pub mod transform;
#[cfg(any(feature = "traversal", doc))]
pub mod traversal;

// Re-export public types and functions
#[cfg(any(feature = "compare", doc))]
pub use compare::TreeDiff;
#[cfg(any(feature = "merge", doc))]
pub use merge::MergeStrategy;
#[cfg(any(feature = "path", doc))]
pub use path::{FlattenedEntry, TreePath};
#[cfg(any(feature = "stats", doc))]
pub use stats::TreeStats;
