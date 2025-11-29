//! Tree construction module.
//!
//! This module provides different ways to build trees, including macros,
//! fluent builders, and incremental construction.

// Declare sub-modules
#[cfg(any(feature = "builder", doc))]
pub mod builder;
#[cfg(any(feature = "incremental", doc))]
pub mod incremental;
#[cfg(any(feature = "macro", doc))]
pub mod macros;

// Re-export public types and functions
#[cfg(any(feature = "builder", doc))]
pub use builder::TreeBuilder;
#[cfg(any(feature = "incremental", doc))]
pub use incremental::IncrementalTree;
// Macro is re-exported at crate root via lib.rs
