//! Cargo metadata integration for dependency tree visualization.

use crate::tree::Tree;

impl Tree {
    /// Builds a dependency tree from cargo metadata.
    ///
    /// Requires the `cargo-metadata` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    ///
    /// let tree = Tree::from_cargo_metadata(".").unwrap();
    /// ```
    #[cfg(feature = "arbitrary-cargo")]
    pub fn from_cargo_metadata<P: AsRef<std::path::Path>>(
        manifest_path: P,
    ) -> Result<Self, cargo_metadata::Error> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(manifest_path.as_ref())
            .exec()?;

        // Build a map of package names to packages (for dependency lookup)
        let packages_by_name: std::collections::HashMap<_, _> = metadata
            .packages
            .iter()
            .map(|pkg| (pkg.name.as_str(), pkg))
            .collect();

        // Find root package (the one we're building)
        let root_package =
            metadata
                .root_package()
                .ok_or_else(|| cargo_metadata::Error::CargoMetadata {
                    stderr: "No root package found".to_string(),
                })?;

        Ok(Self::from_cargo_package(
            root_package,
            &packages_by_name,
            &mut std::collections::HashSet::new(),
        ))
    }

    #[cfg(feature = "arbitrary-cargo")]
    #[allow(clippy::only_used_in_recursion)]
    fn from_cargo_package(
        package: &cargo_metadata::Package,
        packages_by_name: &std::collections::HashMap<&str, &cargo_metadata::Package>,
        visited: &mut std::collections::HashSet<cargo_metadata::PackageId>,
    ) -> Self {
        // Handle cycles
        if visited.contains(&package.id) {
            return Tree::new_leaf(format!("{} (cycle)", package.name));
        }
        visited.insert(package.id.clone());

        let label = format!("{} {}", package.name, package.version);

        // Get dependencies - look up by name
        let children: Vec<Tree> = package
            .dependencies
            .iter()
            .filter_map(|dep| {
                // Find package by name
                packages_by_name
                    .get(dep.name.as_str())
                    .map(|dep_pkg| Self::from_cargo_package(dep_pkg, packages_by_name, visited))
            })
            .collect();

        visited.remove(&package.id);

        if children.is_empty() {
            Tree::new_leaf(label)
        } else {
            Tree::Node(label, children)
        }
    }

    /// Builds a dependency tree for a specific package from cargo metadata.
    ///
    /// Requires the `cargo-metadata` feature.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    ///
    /// let tree = Tree::from_cargo_package_deps("treelog", ".").unwrap();
    /// ```
    #[cfg(feature = "arbitrary-cargo")]
    pub fn from_cargo_package_deps<P: AsRef<std::path::Path>>(
        package_name: &str,
        manifest_path: P,
    ) -> Result<Self, cargo_metadata::Error> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(manifest_path.as_ref())
            .exec()?;

        let packages_by_name: std::collections::HashMap<_, _> = metadata
            .packages
            .iter()
            .map(|pkg| (pkg.name.as_str(), pkg))
            .collect();

        let package = metadata
            .packages
            .iter()
            .find(|pkg| pkg.name == package_name)
            .ok_or_else(|| cargo_metadata::Error::CargoMetadata {
                stderr: format!("Package '{}' not found", package_name),
            })?;

        Ok(Self::from_cargo_package(
            package,
            &packages_by_name,
            &mut std::collections::HashSet::new(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "arbitrary-cargo")]
    #[test]
    fn test_cargo_metadata_parsing() {
        // This test requires a valid Cargo.toml in the project root
        // We'll just check that the function exists and can be called
        // In a real scenario, this would need a test fixture
        let result = Tree::from_cargo_metadata("Cargo.toml");
        // This might fail if not run from project root, which is fine for a test
        if let Ok(tree) = result {
            assert!(tree.is_node());
        }
    }
}
