//! Example: Simulating a file system tree structure.

use treelog::builder::TreeBuilder;

fn main() {
    let mut builder = TreeBuilder::new();

    // Build a file system-like structure
    builder
        .node("home")
        .node("user")
        .node("Documents")
        .leaf("report.pdf")
        .leaf("notes.txt")
        .end()
        .node("Projects")
        .node("myproject")
        .node("src")
        .leaf("main.rs")
        .leaf("lib.rs")
        .end()
        .node("tests")
        .leaf("test.rs")
        .end()
        .leaf("Cargo.toml")
        .end()
        .end()
        .node("Downloads")
        .leaf("file1.zip")
        .leaf("file2.tar.gz")
        .end()
        .end()
        .node("Pictures")
        .leaf("photo1.jpg")
        .leaf("photo2.png")
        .end();

    let tree = builder.build();

    println!("File System Tree:");
    println!("{}", tree.render_to_string());
}
