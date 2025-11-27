//! Example: Visualizing tree-sitter parse trees with treelog.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Tree-sitter Parse Tree Visualization Example\n");

    // Note: This example requires a tree-sitter language to be loaded
    // For demonstration, we show the API usage pattern

    println!("Tree-sitter integration allows you to visualize parse trees.");
    println!("To use this, you need to:");
    println!("1. Load a tree-sitter language (e.g., tree-sitter-rust)");
    println!("2. Parse source code");
    println!("3. Convert the parse tree to a Tree structure\n");

    // Example code structure (this would require actual language loading)
    println!("Example usage pattern:");
    println!(
        r#"
    use treelog::Tree;
    use tree_sitter::Parser;

    let mut parser = Parser::new();
    parser.set_language(language)?;
    let parse_tree = parser.parse(source_code, None)?;
    let tree = Tree::from_tree_sitter(&parse_tree);
    println!("{{}}", tree.render_to_string());
    "#
    );

    // For a working example, you would need to:
    // 1. Add tree-sitter-rust or another language as a dependency
    // 2. Load the language
    // 3. Parse actual source code

    println!("\nThe tree-sitter integration provides:");
    println!("- Tree::from_tree_sitter() - Convert parse tree to Tree");
    println!("- Tree::from_tree_sitter_language() - Parse and convert in one step");

    Ok(())
}
