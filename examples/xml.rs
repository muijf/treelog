//! Example: Visualizing XML/HTML structures with treelog.

use treelog::Tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("XML/HTML Tree Visualization Example\n");

    // Example XML
    let xml = r#"
        <html>
            <head>
                <title>Example</title>
            </head>
            <body>
                <h1>Hello World</h1>
                <p>This is a paragraph.</p>
            </body>
        </html>
    "#;

    let tree = Tree::from_xml(xml)?;
    println!("XML structure:");
    println!("{}", tree.render_to_string());

    println!("\n---\n");

    // Example with attributes
    let xml_with_attrs = r#"
        <root id="1" class="container">
            <child name="first">Content</child>
            <child name="second">More content</child>
        </root>
    "#;

    let tree2 = Tree::from_xml(xml_with_attrs)?;
    println!("XML with attributes:");
    println!("{}", tree2.render_to_string());

    Ok(())
}
