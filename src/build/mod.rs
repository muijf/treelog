//! Tree construction module.
//!
//! This module provides different ways to build trees, including fluent builders
//! and incremental construction.

// Declare sub-modules
#[cfg(any(feature = "builder", doc))]
pub mod builder;
#[cfg(any(feature = "incremental", doc))]
pub mod incremental;

// Re-export public types and functions
#[cfg(any(feature = "builder", doc))]
pub use builder::TreeBuilder;
#[cfg(any(feature = "incremental", doc))]
pub use incremental::IncrementalTree;
