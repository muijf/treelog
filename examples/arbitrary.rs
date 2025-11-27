//! Example: Arbitrary serialization (JSON, YAML, and TOML).
//!
//! This example demonstrates converting arbitrary data structures (not just Tree structures)
//! to Tree format. This is a one-way conversion from arbitrary data to Tree, different from
//! the exact Tree serialization in `serde.rs`.

use treelog::Tree;

fn main() {
    println!("=== Arbitrary Data Conversion Examples ===\n");

    #[cfg(any(
        feature = "arbitrary",
        feature = "arbitrary-json",
        feature = "arbitrary-yaml",
        feature = "arbitrary-toml"
    ))]
    {
        // Example 1: TOML - Converting a Cargo.toml-like structure
        #[cfg(any(feature = "arbitrary", feature = "arbitrary-toml"))]
        {
            println!("--- TOML Example ---");
            let toml_data = r#"
[package]
name = "treelog"
version = "0.0.4"
edition = "2024"
authors = ["Milan de Kruijf"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"

[features]
default = ["builder", "iterator"]
builder = []
"#;

            println!("Original TOML:");
            println!("{}", toml_data);
            println!();

            match Tree::from_arbitrary_toml(toml_data) {
                Ok(tree) => {
                    println!("Tree representation:");
                    println!("{}", tree.render_to_string());
                }
                Err(e) => println!("Error parsing TOML: {}", e),
            }
            println!("\n");
        }

        // Example 2: JSON - Converting a configuration object
        #[cfg(any(feature = "arbitrary", feature = "arbitrary-json"))]
        {
            println!("--- JSON Example ---");
            let json_data = r#"
{
  "app": {
    "name": "MyApp",
    "version": "1.0.0",
    "settings": {
      "debug": true,
      "port": 8080,
      "features": ["auth", "logging", "api"]
    }
  }
}
"#;

            println!("Original JSON:");
            println!("{}", json_data);
            println!();

            match Tree::from_arbitrary_json(json_data) {
                Ok(tree) => {
                    println!("Tree representation:");
                    println!("{}", tree.render_to_string());
                }
                Err(e) => println!("Error parsing JSON: {}", e),
            }
            println!("\n");
        }

        // Example 3: YAML - Converting a configuration file
        #[cfg(any(feature = "arbitrary", feature = "arbitrary-yaml"))]
        {
            println!("--- YAML Example ---");
            let yaml_data = r#"
server:
  host: localhost
  port: 8080
  ssl: true

database:
  type: postgresql
  host: db.example.com
  port: 5432
  credentials:
    username: admin
    password: secret

features:
  - authentication
  - logging
  - monitoring
"#;

            println!("Original YAML:");
            println!("{}", yaml_data);
            println!();

            match Tree::from_arbitrary_yaml(yaml_data) {
                Ok(tree) => {
                    println!("Tree representation:");
                    println!("{}", tree.render_to_string());
                }
                Err(e) => println!("Error parsing YAML: {}", e),
            }
            println!("\n");
        }

        // Example 4: Simple array conversion
        #[cfg(any(feature = "arbitrary", feature = "arbitrary-json"))]
        {
            println!("--- Array Example ---");
            let array_json = r#"[1, 2, 3, "four", true, null]"#;
            println!("Original JSON array: {}", array_json);
            println!();

            match Tree::from_arbitrary_json(array_json) {
                Ok(tree) => {
                    println!("Tree representation:");
                    println!("{}", tree.render_to_string());
                }
                Err(e) => println!("Error parsing JSON: {}", e),
            }
            println!("\n");
        }

        // Example 5: Comparison with exact serialization
        #[cfg(any(feature = "arbitrary", feature = "arbitrary-json"))]
        {
            println!("--- Comparison: Arbitrary vs Exact Serialization ---");
            let json_data = r#"{"name": "test", "value": 42}"#;

            println!("Original JSON: {}", json_data);
            println!();

            // Arbitrary conversion (can handle any JSON)
            if let Ok(arbitrary_tree) = Tree::from_arbitrary_json(json_data) {
                println!("Arbitrary conversion (from_arbitrary_json):");
                println!("{}", arbitrary_tree.render_to_string());
                println!();
            }

            // Exact conversion (only works for Tree structures)
            // This would fail for arbitrary JSON, but works for Tree-serialized JSON
            #[cfg(feature = "serde-json")]
            {
                let tree = Tree::Node(
                    "root".to_string(),
                    vec![
                        Tree::Leaf(vec!["name = \"test\"".to_string()]),
                        Tree::Leaf(vec!["value = 42".to_string()]),
                    ],
                );

                if let Ok(exact_json) = tree.to_json() {
                    println!("Exact serialization (to_json) - only for Tree structures:");
                    println!("{}", exact_json);
                    println!();

                    if let Ok(deserialized) = Tree::from_json(&exact_json) {
                        println!("Deserialized back (from_json):");
                        println!("{}", deserialized.render_to_string());
                    }
                }
            }
        }
    }

    #[cfg(not(any(
        feature = "arbitrary",
        feature = "arbitrary-json",
        feature = "arbitrary-yaml",
        feature = "arbitrary-toml"
    )))]
    {
        println!(
            "Note: Enable the 'arbitrary' feature (or individual 'arbitrary-json', 'arbitrary-yaml', 'arbitrary-toml' features) to see arbitrary serialization examples."
        );
        println!("Run: cargo run --example arbitrary --features arbitrary");
    }
}
