//! Arbitrary data structure conversion support for Tree.
//!
//! This module provides functions to convert arbitrary data structures (JSON, YAML, TOML, XML,
//! filesystem, Git repositories, Rust AST, tree-sitter parse trees, clap commands, cargo
//! metadata, and petgraph graphs) to Tree. This is a one-way conversion from arbitrary data
//! to Tree, separate from the exact Tree serialization in `serde`.

#[cfg(feature = "arbitrary-json")]
mod json;

#[cfg(feature = "arbitrary-yaml")]
mod yaml;

#[cfg(feature = "arbitrary-toml")]
mod toml;

#[cfg(feature = "arbitrary-xml")]
mod xml;

#[cfg(feature = "arbitrary-walkdir")]
mod walkdir;

#[cfg(feature = "arbitrary-git2")]
mod git2;

#[cfg(feature = "arbitrary-syn")]
mod syn;

#[cfg(feature = "arbitrary-tree-sitter")]
mod tree_sitter;

#[cfg(feature = "arbitrary-clap")]
mod clap;

#[cfg(feature = "arbitrary-cargo")]
mod cargo;

#[cfg(feature = "arbitrary-petgraph")]
mod petgraph;
