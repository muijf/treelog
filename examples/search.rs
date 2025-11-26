//! Example demonstrating tree search and query features.

use treelog::Tree;

fn main() {
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
                "src".to_string(), // Duplicate label
                vec![Tree::Leaf(vec!["utils.rs".to_string()])],
            ),
            Tree::Leaf(vec!["README.md".to_string()]),
        ],
    );

    println!("Tree Search Example\n");
    println!("{}", tree.render_to_string());

    println!("\n=== Find First Node ===");
    if let Some(found) = tree.find_node("src") {
        println!("Found node: {}", found.label().unwrap());
    }

    println!("\n=== Find All Nodes ===");
    let all_src_nodes = tree.find_all_nodes("src");
    println!("Found {} nodes with label 'src'", all_src_nodes.len());
    for node in all_src_nodes {
        println!("  - {}", node.label().unwrap());
    }

    println!("\n=== Find Leaf ===");
    if let Some(found) = tree.find_leaf("main.rs") {
        println!("Found leaf containing 'main.rs'");
        if let Tree::Leaf(lines) = found {
            println!("  Lines: {lines:?}");
        }
    }

    println!("\n=== Check if Tree Contains ===");
    println!("Contains 'src': {}", tree.contains("src"));
    println!("Contains 'main.rs': {}", tree.contains("main.rs"));
    println!("Contains 'nonexistent': {}", tree.contains("nonexistent"));

    println!("\n=== Get Path to Node ===");
    if let Some(path) = tree.path_to("src") {
        println!("Path to 'src': {path:?}");
    }
    if let Some(path) = tree.path_to("main.rs") {
        println!("Path to 'main.rs': {path:?}");
    }
}
