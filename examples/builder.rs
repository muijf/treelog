//! Example demonstrating the high-level builder API.

use treelog::build::TreeBuilder;

fn main() {
    // Build a tree using the fluent builder API
    let mut builder = TreeBuilder::new();
    builder
        .node("project")
        .node("src")
        .leaf("main.rs")
        .leaf("lib.rs")
        .end()
        .node("tests")
        .leaf("test1.rs")
        .leaf("test2.rs")
        .end()
        .node("examples")
        .leaf("example1.rs")
        .end()
        .leaf("Cargo.toml")
        .leaf("README.md");

    let tree = builder.build();

    println!("Project Structure (Builder API):");
    println!("{}", tree.render_to_string());
}
