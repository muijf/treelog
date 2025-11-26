//! Example demonstrating the iterator API for line-by-line access.

use treelog::{Tree, iterator::TreeIteratorExt};

fn main() {
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Leaf(vec!["item1".to_string()]),
            Tree::Leaf(vec!["item2".to_string(), "item2-line2".to_string()]),
            Tree::Node(
                "sub".to_string(),
                vec![Tree::Leaf(vec!["subitem".to_string()])],
            ),
        ],
    );

    println!("Tree Lines (Iterator API):");
    println!("{}", "=".repeat(50));
    
    for (i, line) in TreeIteratorExt::lines(&tree).enumerate() {
        println!("Line {}: depth={}, is_last={}", i + 1, line.depth, line.is_last);
        println!("  Prefix: '{}'", line.prefix);
        println!("  Content: '{}'", line.content);
        println!();
    }

    println!("{}", "=".repeat(50));
    println!("\nAll lines as Vec<String>:");
    let lines = tree.to_lines();
    for (i, line) in lines.iter().enumerate() {
        println!("  {}: {}", i + 1, line);
    }
}

