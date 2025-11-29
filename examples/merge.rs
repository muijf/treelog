//! Example demonstrating tree merging features.

use treelog::Tree;
use treelog::ops::merge::MergeStrategy;

fn main() {
    let tree1 = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Node(
                "src".to_string(),
                vec![Tree::Leaf(vec!["main.rs".to_string()])],
            ),
            Tree::Leaf(vec!["README.md".to_string()]),
        ],
    );

    let tree2 = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Node(
                "src".to_string(),
                vec![Tree::Leaf(vec!["lib.rs".to_string()])],
            ),
            Tree::Leaf(vec!["LICENSE".to_string()]),
        ],
    );

    println!("Tree Merging Example\n");

    println!("Tree 1:");
    println!("{}", tree1.render_to_string());
    println!("\nTree 2:");
    println!("{}", tree2.render_to_string());

    println!("\n=== Merge Strategy: Replace ===");
    let merged = tree1.merge(tree2.clone(), MergeStrategy::Replace);
    println!("{}", merged.render_to_string());

    println!("\n=== Merge Strategy: Append ===");
    let merged = tree1.merge(tree2.clone(), MergeStrategy::Append);
    println!("{}", merged.render_to_string());

    println!("\n=== Merge Strategy: MergeByLabel ===");
    let merged = tree1.merge(tree2, MergeStrategy::MergeByLabel);
    println!("{}", merged.render_to_string());
}
