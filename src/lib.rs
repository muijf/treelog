#![doc = include_str!("../README.md")]

#[cfg(any(
    feature = "arbitrary-json",
    feature = "arbitrary-yaml",
    feature = "arbitrary-toml",
    feature = "arbitrary-xml",
    feature = "arbitrary-walkdir",
    feature = "arbitrary-git2",
    feature = "arbitrary-syn",
    feature = "arbitrary-tree-sitter",
    feature = "arbitrary-clap",
    feature = "arbitrary-cargo",
    feature = "arbitrary-petgraph",
    doc
))]
pub mod arbitrary;
#[cfg(any(feature = "builder", doc))]
pub mod builder;
#[cfg(any(feature = "compare", doc))]
pub mod compare;
pub mod config;
#[cfg(any(feature = "export", doc))]
pub mod export;
#[cfg(any(feature = "indicatif", doc))]
pub mod indicatif;
#[cfg(any(feature = "iterator", doc))]
pub mod iterator;
mod level;
#[cfg(any(feature = "macro", doc))]
mod macros;
#[cfg(any(feature = "merge", doc))]
pub mod merge;
#[cfg(any(feature = "path", doc))]
pub mod path;
mod prefix;
pub mod renderer;
#[cfg(any(feature = "search", doc))]
pub mod search;
#[cfg(any(
    feature = "serde-json",
    feature = "serde-yaml",
    feature = "serde-toml",
    feature = "serde-ron",
    doc
))]
pub mod serde;
#[cfg(any(feature = "sort", doc))]
pub mod sort;
#[cfg(any(feature = "stats", doc))]
pub mod stats;
pub mod style;
#[cfg(any(feature = "transform", doc))]
pub mod transform;
#[cfg(any(feature = "traversal", doc))]
pub mod traversal;
pub mod tree;
pub mod utils;

// Re-export main types
pub use config::RenderConfig;
#[cfg(any(feature = "iterator", doc))]
pub use iterator::{Line, TreeIteratorExt};
pub use level::LevelPath;
#[cfg(any(feature = "stats", doc))]
pub use stats::TreeStats;
pub use style::{StyleConfig, TreeStyle};
pub use tree::Tree;

#[cfg(any(feature = "indicatif", doc))]
pub use indicatif::TreeProgressManager;

// Re-export renderer functions
pub use renderer::{
    render_to_string, render_to_string_with_config, write_tree, write_tree_with_config,
};

// Re-export prefix functions
pub use prefix::{compute_prefix, compute_second_line_prefix};

/// Extension methods for Tree that provide convenient rendering.
impl Tree {
    /// Renders this tree to a String using the default configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let output = tree.render_to_string();
    /// ```
    pub fn render_to_string(&self) -> String {
        render_to_string(self)
    }

    /// Renders this tree to a String using a custom configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::{Tree, TreeStyle, RenderConfig};
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let config = RenderConfig::default().with_style(TreeStyle::Ascii);
    /// let output = tree.render_to_string_with_config(&config);
    /// ```
    pub fn render_to_string_with_config(&self, config: &RenderConfig) -> String {
        render_to_string_with_config(self, config)
    }

    /// Renders this tree to a writer using the default configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    /// use std::fmt::Write;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let mut output = String::new();
    /// tree.render_to_writer(&mut output).unwrap();
    /// ```
    pub fn render_to_writer(&self, writer: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write_tree(writer, self)
    }

    /// Renders this tree to a writer using a custom configuration.
    pub fn render_to_writer_with_config(
        &self,
        writer: &mut dyn std::fmt::Write,
        config: &RenderConfig,
    ) -> std::fmt::Result {
        write_tree_with_config(writer, self, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_rendering() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let output = tree.render_to_string();
        assert!(output.contains("root"));
        assert!(output.contains("item"));
    }

    #[cfg(feature = "builder")]
    #[test]
    fn test_builder_api() {
        use crate::builder::TreeBuilder;
        let mut builder = TreeBuilder::new();
        builder.node("root").leaf("item");
        let tree = builder.build();
        assert!(tree.is_node());
    }

    #[cfg(feature = "iterator")]
    #[test]
    fn test_iterator_api() {
        use crate::iterator::TreeIteratorExt;
        let tree = Tree::Node(
            "root".to_string(),
            vec![Tree::Leaf(vec!["item".to_string()])],
        );
        let lines: Vec<_> = TreeIteratorExt::lines(&tree).collect();
        assert!(!lines.is_empty());
    }
}
