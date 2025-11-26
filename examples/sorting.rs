//! Example demonstrating tree sorting features.

use std::cmp::Ordering;
use treelog::Tree;

fn main() {
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Node(
                "zebra".to_string(),
                vec![Tree::Leaf(vec!["z1".to_string()])],
            ),
            Tree::Node(
                "apple".to_string(),
                vec![Tree::Leaf(vec!["a1".to_string()])],
            ),
            Tree::Node(
                "banana".to_string(),
                vec![Tree::Leaf(vec!["b1".to_string()])],
            ),
            Tree::Leaf(vec!["leaf1".to_string()]),
        ],
    );

    println!("Tree Sorting Example\n");
    println!("Original tree (unsorted):");
    println!("{}", tree.render_to_string());

    println!("\n=== Sort by Label ===");
    let mut tree2 = tree.clone();
    tree2.sort_by_label();
    println!("{}", tree2.render_to_string());

    println!("\n=== Sort by Depth (deepest first) ===");
    let mut tree3 = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Leaf(vec!["shallow".to_string()]),
            Tree::Node(
                "deep".to_string(),
                vec![Tree::Node(
                    "deeper".to_string(),
                    vec![Tree::Leaf(vec!["deepest".to_string()])],
                )],
            ),
        ],
    );
    println!("Before sorting:");
    println!("{}", tree3.render_to_string());
    tree3.sort_by_depth(true);
    println!("\nAfter sorting (deepest first):");
    println!("{}", tree3.render_to_string());

    println!("\n=== Custom Sort ===");
    let mut tree4 = tree.clone();
    let mut compare = |a: &Tree, b: &Tree| match (a, b) {
        (Tree::Node(label_a, _), Tree::Node(label_b, _)) => label_a.len().cmp(&label_b.len()),
        (Tree::Leaf(_), Tree::Node(_, _)) => Ordering::Less,
        (Tree::Node(_, _), Tree::Leaf(_)) => Ordering::Greater,
        _ => Ordering::Equal,
    };
    tree4.sort_children(&mut compare);
    println!("Sorted by label length:");
    println!("{}", tree4.render_to_string());
}
