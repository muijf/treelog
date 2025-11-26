//! Basic example demonstrating simple tree construction and rendering.

use treelog::{Tree, renderer::write_tree};

fn main() {
    // Create a simple tree
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Leaf(vec!["item1".to_string()]),
            Tree::Leaf(vec!["item2".to_string()]),
            Tree::Node(
                "subdirectory".to_string(),
                vec![
                    Tree::Leaf(vec!["subitem1".to_string()]),
                    Tree::Leaf(vec!["subitem2".to_string()]),
                ],
            ),
        ],
    );

    // Render to string
    let mut output = String::new();
    write_tree(&mut output, &tree).unwrap();

    println!("Basic Tree:");
    println!("{}", output);
}
