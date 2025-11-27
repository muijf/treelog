//! Example: RON serialization with treelog.

use treelog::Tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RON Serialization Example\n");

    // Create a tree
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Leaf(vec!["item1".to_string()]),
            Tree::Node(
                "sub".to_string(),
                vec![
                    Tree::Leaf(vec!["subitem1".to_string()]),
                    Tree::Leaf(vec!["subitem2".to_string()]),
                ],
            ),
        ],
    );

    println!("Original tree:");
    println!("{}", tree.render_to_string());

    println!("\n---\n");

    // Serialize to RON
    let ron = tree.to_ron()?;
    println!("RON representation:");
    println!("{}", ron);

    println!("\n---\n");

    // Serialize to pretty RON
    let ron_pretty = tree.to_ron_pretty()?;
    println!("Pretty RON representation:");
    println!("{}", ron_pretty);

    println!("\n---\n");

    // Deserialize back
    let deserialized = Tree::from_ron(&ron)?;
    println!("Deserialized tree:");
    println!("{}", deserialized.render_to_string());

    assert_eq!(tree, deserialized);
    println!("\n✓ Round-trip successful!");

    Ok(())
}
