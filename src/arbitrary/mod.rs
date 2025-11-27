//! Arbitrary serialization and deserialization support for Tree.
//!
//! This module provides functions to convert arbitrary TOML/YAML/JSON/XML data structures
//! to Tree and vice versa, separate from the exact Tree serialization in `serde`.

#[cfg(feature = "arbitrary-json")]
mod json;

#[cfg(feature = "arbitrary-yaml")]
mod yaml;

#[cfg(feature = "arbitrary-toml")]
mod toml;

#[cfg(feature = "arbitrary-xml")]
mod xml;
