//! Example demonstrating tree comparison features.

use treelog::Tree;

fn main() {
    let tree1 = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Node("a".to_string(), vec![Tree::Leaf(vec!["a1".to_string()])]),
            Tree::Leaf(vec!["b".to_string()]),
        ],
    );

    let tree2 = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Node("x".to_string(), vec![Tree::Leaf(vec!["x1".to_string()])]),
            Tree::Leaf(vec!["y".to_string()]),
        ],
    );

    let tree3 = Tree::Node(
        "different".to_string(),
        vec![
            Tree::Node("a".to_string(), vec![Tree::Leaf(vec!["a1".to_string()])]),
            Tree::Leaf(vec!["b".to_string()]),
        ],
    );

    println!("Tree Comparison Example\n");

    println!("Tree 1:");
    println!("{}", tree1.render_to_string());
    println!("\nTree 2:");
    println!("{}", tree2.render_to_string());
    println!("\nTree 3:");
    println!("{}", tree3.render_to_string());

    println!("\n=== Compare Structure ===");
    println!(
        "Tree 1 and Tree 2 have same structure: {}",
        tree1.eq_structure(&tree2)
    );
    println!(
        "Tree 1 and Tree 3 have same structure: {}",
        tree1.eq_structure(&tree3)
    );

    println!("\n=== Compute Differences ===");
    let diffs = tree1.diff(&tree2);
    println!("Differences between Tree 1 and Tree 2:");
    for diff in diffs {
        match diff {
            treelog::compare::TreeDiff::OnlyInFirst { path, content } => {
                println!("  Only in first at path {path:?}: {content}");
            }
            treelog::compare::TreeDiff::OnlyInSecond { path, content } => {
                println!("  Only in second at path {path:?}: {content}");
            }
            treelog::compare::TreeDiff::DifferentContent {
                path,
                first,
                second,
            } => {
                println!("  Different at path {path:?}: '{first}' vs '{second}'");
            }
        }
    }

    println!("\n=== Check if Subtree ===");
    let subtree = Tree::Node("a".to_string(), vec![Tree::Leaf(vec!["a1".to_string()])]);
    println!("Is subtree in Tree 1: {}", subtree.is_subtree_of(&tree1));
    println!("Is subtree in Tree 2: {}", subtree.is_subtree_of(&tree2));
}
