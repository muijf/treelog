//! Example demonstrating tree traversal iterators.

use treelog::Tree;

fn main() {
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Node(
                "a".to_string(),
                vec![
                    Tree::Leaf(vec!["a1".to_string()]),
                    Tree::Leaf(vec!["a2".to_string()]),
                ],
            ),
            Tree::Node("b".to_string(), vec![Tree::Leaf(vec!["b1".to_string()])]),
            Tree::Leaf(vec!["c".to_string()]),
        ],
    );

    println!("Tree Traversal Example\n");
    println!("{}", tree.render_to_string());

    println!("\n=== Pre-order Traversal (root, then children) ===");
    for node in tree.pre_order() {
        match node {
            Tree::Node(label, _) => println!("Node: {label}"),
            Tree::Leaf(lines) => println!("Leaf: {}", lines[0]),
        }
    }

    println!("\n=== Post-order Traversal (children, then root) ===");
    for node in tree.post_order() {
        match node {
            Tree::Node(label, _) => println!("Node: {label}"),
            Tree::Leaf(lines) => println!("Leaf: {}", lines[0]),
        }
    }

    println!("\n=== Level-order Traversal (breadth-first) ===");
    for node in tree.level_order() {
        match node {
            Tree::Node(label, _) => println!("Node: {label}"),
            Tree::Leaf(lines) => println!("Leaf: {}", lines[0]),
        }
    }

    println!("\n=== Iterate over Nodes Only ===");
    for node in tree.nodes() {
        println!("Node: {}", node.label().unwrap());
    }

    println!("\n=== Iterate over Leaves Only ===");
    for leaf in tree.leaves() {
        println!("Leaf: {:?}", leaf.lines());
    }
}
