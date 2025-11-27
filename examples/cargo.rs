//! Example: Visualizing Cargo dependency trees.

use treelog::Tree;

fn main() {
    #[cfg(feature = "arbitrary-cargo")]
    {
        println!("=== Cargo Dependency Tree ===");
        match Tree::from_cargo_metadata("Cargo.toml") {
            Ok(tree) => {
                println!("{}", tree.render_to_string());
            }
            Err(e) => {
                println!("Error reading cargo metadata: {}", e);
                println!("Make sure you're running this from a Rust project directory.");
            }
        }

        println!("\n=== Specific Package Dependencies ===");
        // Try to get dependencies for this package
        match Tree::from_cargo_package_deps("treelog", "Cargo.toml") {
            Ok(tree) => {
                println!("{}", tree.render_to_string());
            }
            Err(e) => {
                println!("Error reading package dependencies: {}", e);
            }
        }
    }

    #[cfg(not(feature = "cargo-metadata"))]
    {
        println!("Note: Enable 'cargo-metadata' feature to use cargo dependency visualization.");
        println!("Example usage:");
        println!("  let tree = Tree::from_cargo_metadata(\"Cargo.toml\").unwrap();");
    }
}
