//! Example demonstrating customization options: styles, formatters, and configurations.

use treelog::{RenderConfig, StyleConfig, Tree, TreeStyle};

fn main() {
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Leaf(vec!["item1".to_string()]),
            Tree::Node(
                "sub".to_string(),
                vec![Tree::Leaf(vec!["subitem".to_string()])],
            ),
        ],
    );

    println!("1. Unicode Style (default):");
    println!("{}", tree.render_to_string());

    println!("\n2. ASCII Style:");
    let ascii_config = RenderConfig::default().with_style(TreeStyle::Ascii);
    println!("{}", tree.render_to_string_with_config(&ascii_config));

    println!("\n3. Custom Style:");
    let custom_style = StyleConfig::custom(">", "<", "|", " ");
    let custom_config = RenderConfig::default().with_style(custom_style);
    println!("{}", tree.render_to_string_with_config(&custom_config));

    #[cfg(feature = "formatters")]
    {
        println!("\n4. Custom Formatters:");
        let formatted_config = RenderConfig::default()
            .with_node_formatter(|label| format!("[DIR] {}", label))
            .with_leaf_formatter(|line| format!("[FILE] {}", line));
        println!("{}", tree.render_to_string_with_config(&formatted_config));

        println!("\n5. Combined Customization:");
        let combined_config = RenderConfig::default()
            .with_style(TreeStyle::Ascii)
            .with_node_formatter(|label| format!("[{}]", label.to_uppercase()))
            .with_leaf_formatter(|line| format!("- {}", line));
        println!("{}", tree.render_to_string_with_config(&combined_config));
    }
    #[cfg(not(feature = "formatters"))]
    {
        println!("\n4. Custom Formatters: (requires 'formatters' feature)");
        println!("   Enable with: cargo run --example customization --features formatters");
    }
}
