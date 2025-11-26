//! Example demonstrating tree transformation features.

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

    println!("Tree Transformation Example\n");
    println!("Original tree:");
    println!("{}", tree.render_to_string());

    println!("\n=== Map Node Labels ===");
    let transformed = tree.map_nodes(|label| format!("[{label}]"));
    println!("{}", transformed.render_to_string());

    println!("\n=== Map Leaf Lines ===");
    let transformed = tree.map_leaves(|line| format!("- {line}"));
    println!("{}", transformed.render_to_string());

    println!("\n=== Filter Tree ===");
    let filtered = tree.filter(|t| match t {
        Tree::Leaf(lines) => lines.iter().any(|l| l.contains("main")),
        Tree::Node(_, _) => true,
    });
    if let Some(filtered_tree) = filtered {
        println!("Filtered tree (keeping only leaves with 'main'):");
        println!("{}", filtered_tree.render_to_string());
    }

    println!("\n=== Prune Tree ===");
    let pruned = tree.prune(|t| match t {
        Tree::Leaf(lines) => lines.iter().any(|l| l.contains("README")),
        Tree::Node(_, _) => false,
    });
    if let Some(pruned_tree) = pruned {
        println!("Pruned tree (removed leaves with 'README'):");
        println!("{}", pruned_tree.render_to_string());
    }
}
