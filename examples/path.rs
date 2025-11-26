//! Example demonstrating tree path utilities.

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
            Tree::Leaf(vec!["README.md".to_string()]),
        ],
    );

    println!("Tree Path Utilities Example\n");
    println!("{}", tree.render_to_string());

    println!("\n=== Get Path to Node ===");
    let child = &tree.children().unwrap()[0];
    if let Some(path) = tree.get_path(child) {
        println!("Path to 'src' node: {path:?}");
    }

    println!("\n=== Get Node by Path ===");
    if let Some(node) = tree.get_by_path(&[0]) {
        println!("Node at path [0]: {}", node.label().unwrap());
    }
    if let Some(Tree::Leaf(lines)) = tree.get_by_path(&[0, 0]) {
        println!("Leaf at path [0, 0]: {}", lines[0]);
    }

    println!("\n=== Modify Node by Path ===");
    let mut tree2 = tree.clone();
    if let Some(Tree::Node(label, _)) = tree2.get_by_path_mut(&[0]) {
        *label = "source".to_string();
    }
    println!("After modifying node at path [0]:");
    println!("{}", tree2.render_to_string());

    println!("\n=== Flatten Tree ===");
    let flattened = tree.flatten();
    println!("Flattened tree entries:");
    for entry in flattened {
        println!(
            "  Path: {:?}, Content: '{}', Is Node: {}",
            entry.path, entry.content, entry.is_node
        );
    }
}
