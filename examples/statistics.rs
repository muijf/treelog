//! Example demonstrating tree statistics and analysis features.

use treelog::Tree;

fn main() {
    // Create a complex tree structure
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Node(
                "src".to_string(),
                vec![
                    Tree::Leaf(vec!["main.rs".to_string()]),
                    Tree::Leaf(vec!["lib.rs".to_string()]),
                ],
            ),
            Tree::Node(
                "tests".to_string(),
                vec![Tree::Leaf(vec![
                    "test1.rs".to_string(),
                    "test2.rs".to_string(),
                ])],
            ),
            Tree::Leaf(vec!["README.md".to_string()]),
        ],
    );

    println!("Tree Statistics Example\n");
    println!("{}", tree.render_to_string());

    // Get individual statistics
    println!("\n=== Individual Statistics ===");
    println!("Depth: {}", tree.depth());
    println!("Width: {}", tree.width());
    println!("Node count: {}", tree.node_count());
    println!("Leaf count: {}", tree.leaf_count());
    println!("Total lines: {}", tree.total_lines());

    // Get all statistics at once
    println!("\n=== All Statistics ===");
    let stats = tree.stats();
    println!("{stats:#?}");
}
