//! Example: Visualizing tree-sitter parse trees with treelog.

use tree_sitter::Parser;
use treelog::Tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Tree-sitter Parse Tree Visualization Example\n");

    // Load the Rust language
    let language: tree_sitter::Language = tree_sitter_rust::LANGUAGE.into();

    // Example Rust code to parse
    let source_code = r#"
        fn main() {
            let x = 42;
            println!("Hello, world!");
        }
    "#;

    println!("Parsing Rust code:");
    println!("{}", source_code);
    println!("\n---\n");

    // Method 1: Parse and convert separately
    println!("Method 1: Using Tree::from_tree_sitter()");
    let mut parser = Parser::new();
    parser.set_language(&language)?;
    let parse_tree = parser.parse(source_code, None).ok_or("Parse failed")?;
    let tree = Tree::from_tree_sitter(&parse_tree);
    println!("{}", tree.render_to_string());

    println!("\n---\n");

    // Method 2: Parse and convert in one step
    println!("Method 2: Using Tree::from_tree_sitter_language()");
    let tree2 = Tree::from_tree_sitter_language(source_code, language)?;
    println!("{}", tree2.render_to_string());

    Ok(())
}
