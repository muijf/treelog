//! Example: Visualizing Rust AST with treelog.

use treelog::Tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust AST Visualization Example\n");

    // Parse a Rust file
    let tree = Tree::from_syn_file("src/lib.rs")?;
    println!("Rust file AST structure:");
    println!("{}", tree.render_to_string());

    println!("\n---\n");

    // Parse Rust code directly
    let code = r#"
        struct Person {
            name: String,
            age: u32,
        }

        impl Person {
            fn new(name: String, age: u32) -> Self {
                Self { name, age }
            }
        }
    "#;

    let ast = syn::parse_file(code)?;
    let tree2 = Tree::from_syn_file_ast(&ast);
    println!("Rust code AST structure:");
    println!("{}", tree2.render_to_string());

    println!("\n---\n");

    // Parse individual items
    let item: syn::Item = syn::parse_quote! {
        enum Color {
            Red,
            Green,
            Blue,
        }
    };

    let tree3 = Tree::from_syn_item(&item);
    println!("Individual item AST:");
    println!("{}", tree3.render_to_string());

    Ok(())
}
