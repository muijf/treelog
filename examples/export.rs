//! Example demonstrating tree export to various formats.

use std::fs;
use treelog::Tree;

fn main() {
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Node(
                "src".to_string(),
                vec![
                    Tree::Leaf(vec!["main.rs".to_string()]),
                    Tree::Leaf(vec!["lib.rs".to_string()]),
                ],
            ),
            Tree::Leaf(vec!["README.md".to_string()]),
        ],
    );

    println!("Tree Export Example\n");
    println!("Original tree:");
    println!("{}", tree.render_to_string());

    // Create exports directory if it doesn't exist
    fs::create_dir_all("exports").expect("Failed to create exports directory");

    println!("\n=== Export to HTML ===");
    let html = tree.to_html();
    println!("HTML output (first 200 chars):");
    let preview_len = html.len().min(200);
    println!("{}...", &html[..preview_len]);
    fs::write("exports/tree.html", html).expect("Failed to write HTML file");
    println!("Full HTML written to exports/tree.html");

    println!("\n=== Export to SVG ===");
    let svg = tree.to_svg();
    println!("SVG output (first 200 chars):");
    println!("{}...", &svg[..svg.len().min(200)]);
    fs::write("exports/tree.svg", svg).expect("Failed to write SVG file");
    println!("Full SVG written to exports/tree.svg");

    println!("\n=== Export to DOT (Graphviz) ===");
    let dot = tree.to_dot();
    println!("DOT output:");
    println!("{dot}");
    fs::write("exports/tree.dot", dot).expect("Failed to write DOT file");
    println!("DOT file written to exports/tree.dot");
    println!("\nYou can render the DOT file with: dot -Tpng exports/tree.dot -o exports/tree.png");
}
