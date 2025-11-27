//! Utility functions for CLI operations.

use super::args::{Cli, OutputFormat};
use std::io::{self, Read};

#[allow(unused_variables)]
pub fn read_tree(input: &str) -> Result<treelog::Tree, Box<dyn std::error::Error>> {
    let content = read_file_or_stdin(input)?;

    // Try to deserialize from JSON first
    #[cfg(feature = "serde-json")]
    if let Ok(tree) = treelog::Tree::from_json(&content) {
        return Ok(tree);
    }

    // Try YAML
    #[cfg(feature = "serde-yaml")]
    if let Ok(tree) = treelog::Tree::from_yaml(&content) {
        return Ok(tree);
    }

    // Try TOML
    #[cfg(feature = "serde-toml")]
    if let Ok(tree) = treelog::Tree::from_toml(&content) {
        return Ok(tree);
    }

    // Try RON
    #[cfg(feature = "serde-ron")]
    if let Ok(tree) = treelog::Tree::from_ron(&content) {
        return Ok(tree);
    }

    #[cfg(not(any(
        feature = "serde-json",
        feature = "serde-yaml",
        feature = "serde-toml",
        feature = "serde-ron"
    )))]
    let _ = content; // Suppress unused variable warning when no features enabled

    Err("Could not parse tree. Ensure the input is valid JSON, YAML, TOML, or RON, or enable the appropriate feature.".into())
}

pub fn read_file_or_stdin(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if path == "-" {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        Ok(content)
    } else {
        std::fs::read_to_string(path).map_err(|e| e.into())
    }
}

pub fn output_tree(tree: &treelog::Tree, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let config = build_render_config(cli)?;
    let output = match &cli.format {
        OutputFormat::Text => tree.render_to_string_with_config(&config),
        #[cfg(feature = "export")]
        OutputFormat::Html => tree.to_html(),
        #[cfg(feature = "export")]
        OutputFormat::Svg => tree.to_svg(),
        #[cfg(feature = "export")]
        OutputFormat::Dot => tree.to_dot(),
        #[cfg(feature = "serde-json")]
        OutputFormat::Json => tree
            .to_json_pretty()
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))?,
        #[cfg(feature = "serde-yaml")]
        OutputFormat::Yaml => tree
            .to_yaml()
            .map_err(|e| format!("Failed to serialize to YAML: {}", e))?,
        #[cfg(feature = "serde-toml")]
        OutputFormat::Toml => tree
            .to_toml()
            .map_err(|e| format!("Failed to serialize to TOML: {}", e))?,
        #[cfg(feature = "serde-ron")]
        OutputFormat::Ron => tree
            .to_ron_pretty()
            .map_err(|e| format!("Failed to serialize to RON: {}", e))?,
    };

    if let Some(output_path) = &cli.output {
        if output_path == "-" {
            print!("{}", output);
        } else {
            std::fs::write(output_path, output)?;
        }
    } else {
        print!("{}", output);
    }

    Ok(())
}

pub fn build_render_config(cli: &Cli) -> Result<treelog::RenderConfig, Box<dyn std::error::Error>> {
    use treelog::{RenderConfig, StyleConfig};

    let mut config = RenderConfig::default();

    // Set style
    if let Some(custom) = &cli.custom_style {
        let parts: Vec<&str> = custom.split(',').collect();
        if parts.len() != 4 {
            return Err(
                "Custom style must have 4 comma-separated values: branch,last,vertical,empty"
                    .into(),
            );
        }
        config = config.with_style(StyleConfig::custom(
            parts[0].trim(),
            parts[1].trim(),
            parts[2].trim(),
            parts[3].trim(),
        ));
    } else {
        // cli.style is already a treelog::TreeStyle (Unicode, Ascii, or Box)
        // Custom variant is skipped by ValueEnum, so we can safely use it
        config = config.with_style(cli.style.clone());
    }

    // Set colors
    #[cfg(feature = "color")]
    {
        if cli.color && !cli.no_color {
            config = config.with_colors(true);
        } else if cli.no_color {
            config = config.with_colors(false);
        }
    }

    Ok(config)
}
