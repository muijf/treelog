//! File system tree building using walkdir.

use crate::tree::Tree;
use std::path::Path;

impl Tree {
    /// Builds a tree from a directory structure.
    ///
    /// Requires the `walkdir` feature.
    ///
    /// Directories become nodes, and files become leaves.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::from_dir(".").unwrap();
    /// ```
    #[cfg(feature = "walkdir")]
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self, walkdir::Error> {
        let path = path.as_ref();
        let mut tree = Tree::new_node(
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(".")
                .to_string(),
        );

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry_result in walkdir::WalkDir::new(path)
            .max_depth(1)
            .min_depth(1)
            .into_iter()
        {
            let entry = entry_result?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path.to_path_buf());
            } else if path.is_file() {
                files.push(path.to_path_buf());
            }
        }

        // Sort for consistent output
        dirs.sort();
        files.sort();

        // Add directories as nodes (recursively)
        for dir in dirs {
            if let Ok(subtree) = Self::from_dir(&dir) {
                tree.add_child(subtree);
            }
        }

        // Add files as leaves
        for file in files {
            if let Some(name) = file.file_name().and_then(|n| n.to_str()) {
                tree.add_child(Tree::new_leaf(name.to_string()));
            }
        }

        Ok(tree)
    }

    /// Builds a tree from a directory structure with a maximum depth.
    ///
    /// Requires the `walkdir` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    ///
    /// let tree = Tree::from_dir_max_depth(".", 2).unwrap();
    /// ```
    #[cfg(feature = "walkdir")]
    pub fn from_dir_max_depth<P: AsRef<Path>>(
        path: P,
        max_depth: usize,
    ) -> Result<Self, walkdir::Error> {
        let path = path.as_ref();
        let mut tree = Tree::new_node(
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(".")
                .to_string(),
        );

        if max_depth == 0 {
            return Ok(tree);
        }

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry_result in walkdir::WalkDir::new(path)
            .max_depth(1)
            .min_depth(1)
            .into_iter()
        {
            let entry = entry_result?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path.to_path_buf());
            } else if path.is_file() {
                files.push(path.to_path_buf());
            }
        }

        // Sort for consistent output
        dirs.sort();
        files.sort();

        // Add directories as nodes (recursively with reduced depth)
        for dir in dirs {
            if let Ok(subtree) = Self::from_dir_max_depth(&dir, max_depth - 1) {
                tree.add_child(subtree);
            }
        }

        // Add files as leaves
        for file in files {
            if let Some(name) = file.file_name().and_then(|n| n.to_str()) {
                tree.add_child(Tree::new_leaf(name.to_string()));
            }
        }

        Ok(tree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "walkdir")]
    #[test]
    fn test_from_dir_current() {
        // Test with current directory (should always exist)
        let tree = Tree::from_dir(".");
        assert!(tree.is_ok());
        let tree = tree.unwrap();
        assert!(tree.is_node());
    }

    #[cfg(feature = "walkdir")]
    #[test]
    fn test_from_dir_max_depth() {
        // Test with current directory and max depth
        let tree = Tree::from_dir_max_depth(".", 1);
        assert!(tree.is_ok());
        let tree = tree.unwrap();
        assert!(tree.is_node());
    }
}
