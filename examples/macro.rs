//! Example demonstrating the tree! macro DSL for constructing trees.

use treelog::tree;

fn main() {
    // Build a tree using the macro DSL
    // This is much more concise and readable than manual construction
    let tree = tree! {
        project {
            src {
                "main.rs",
                "lib.rs"
            },
            tests {
                "test1.rs",
                "test2.rs"
            },
            examples {
                "example1.rs"
            },
            "Cargo.toml",
            "README.md"
        }
    };

    println!("Project Structure (Macro DSL):");
    println!("{}", tree.render_to_string());

    // Example with string node names
    let file_tree = tree! {
        "home" {
            "user" {
                "Documents" {
                    "report.pdf",
                    "notes.txt"
                },
                "Projects" {
                    "myproject" {
                        "src" {
                            "main.rs",
                            "lib.rs"
                        },
                        "tests" {
                            "test.rs"
                        },
                        "Cargo.toml"
                    }
                },
                "Downloads" {
                    "file1.zip",
                    "file2.tar.gz"
                }
            }
        }
    };

    println!("\nFile System Tree (Macro DSL):");
    println!("{}", file_tree.render_to_string());

    // Example with mixed syntax (identifiers and strings)
    let mixed_tree = tree! {
        root {
            item1,
            "item2",
            sub {
                "subitem1",
                subitem2
            }
        }
    };

    println!("\nMixed Syntax Example:");
    println!("{}", mixed_tree.render_to_string());
}
