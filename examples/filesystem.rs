//! Example: Building trees from file system directories using walkdir.

use treelog::Tree;

fn main() {
    #[cfg(feature = "walkdir")]
    {
        println!("=== File System Tree (Current Directory) ===");
        match Tree::from_dir(".") {
            Ok(tree) => {
                println!("{}", tree.render_to_string());
            }
            Err(e) => {
                println!("Error reading directory: {}", e);
            }
        }

        println!("\n=== File System Tree (Max Depth 2) ===");
        match Tree::from_dir_max_depth(".", 2) {
            Ok(tree) => {
                println!("{}", tree.render_to_string());
            }
            Err(e) => {
                println!("Error reading directory: {}", e);
            }
        }

        // Try to read src directory if it exists
        if std::path::Path::new("src").exists() {
            println!("\n=== File System Tree (src directory) ===");
            match Tree::from_dir("src") {
                Ok(tree) => {
                    println!("{}", tree.render_to_string());
                }
                Err(e) => {
                    println!("Error reading src directory: {}", e);
                }
            }
        }
    }

    #[cfg(not(feature = "walkdir"))]
    {
        println!("Note: Enable 'walkdir' feature to use file system tree building.");
        println!("Example usage:");
        println!("  let tree = Tree::from_dir(\".\").unwrap();");
    }
}
