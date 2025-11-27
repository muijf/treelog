//! Example: Serialization and deserialization with serde (JSON, YAML, and TOML).

use treelog::Tree;

fn main() {
    // Create a sample tree
    let tree = Tree::Node(
        "root".to_string(),
        vec![
            Tree::Leaf(vec!["item1".to_string()]),
            Tree::Node(
                "subdirectory".to_string(),
                vec![
                    Tree::Leaf(vec!["subitem1".to_string()]),
                    Tree::Leaf(vec!["subitem2".to_string()]),
                ],
            ),
        ],
    );

    println!("Original Tree:");
    println!("{}", tree.render_to_string());
    println!();

    #[cfg(feature = "json")]
    {
        println!("=== JSON Serialization ===");
        match tree.to_json() {
            Ok(json) => {
                println!("JSON (compact):\n{}", json);
                println!();
            }
            Err(e) => println!("Error serializing to JSON: {}", e),
        }

        match tree.to_json_pretty() {
            Ok(json) => {
                println!("JSON (pretty):\n{}", json);
                println!();
            }
            Err(e) => println!("Error serializing to JSON: {}", e),
        }

        // Roundtrip: serialize then deserialize
        if let Ok(json) = tree.to_json() {
            match Tree::from_json(&json) {
                Ok(deserialized) => {
                    println!("Roundtrip (serialize -> deserialize):");
                    println!("{}", deserialized.render_to_string());
                    println!();
                }
                Err(e) => println!("Error deserializing from JSON: {}", e),
            }
        }
    }

    #[cfg(feature = "yaml")]
    {
        println!("=== YAML Serialization ===");
        match tree.to_yaml() {
            Ok(yaml) => {
                println!("YAML:\n{}", yaml);
                println!();
            }
            Err(e) => println!("Error serializing to YAML: {}", e),
        }

        // Roundtrip: serialize then deserialize
        if let Ok(yaml) = tree.to_yaml() {
            match Tree::from_yaml(&yaml) {
                Ok(deserialized) => {
                    println!("Roundtrip (serialize -> deserialize):");
                    println!("{}", deserialized.render_to_string());
                }
                Err(e) => println!("Error deserializing from YAML: {}", e),
            }
        }
    }

    #[cfg(all(feature = "toml", feature = "serde"))]
    {
        println!("=== TOML Serialization (serde-based) ===");
        match tree.to_toml() {
            Ok(toml) => {
                println!("TOML:\n{}", toml);
                println!();
            }
            Err(e) => println!("Error serializing to TOML: {}", e),
        }

        match tree.to_toml_pretty() {
            Ok(toml) => {
                println!("TOML (pretty):\n{}", toml);
                println!();
            }
            Err(e) => println!("Error serializing to TOML: {}", e),
        }

        // Roundtrip: serialize then deserialize
        if let Ok(toml) = tree.to_toml() {
            match Tree::from_toml(&toml) {
                Ok(deserialized) => {
                    println!("Roundtrip (serialize -> deserialize):");
                    println!("{}", deserialized.render_to_string());
                }
                Err(e) => println!("Error deserializing from TOML: {}", e),
            }
        }
    }

    #[cfg(not(any(
        feature = "json",
        feature = "yaml",
        all(feature = "toml", feature = "serde")
    )))]
    {
        println!(
            "Note: Enable 'json', 'yaml', or both 'toml' and 'serde' features to see serialization examples."
        );
    }
}
