//! Example demonstrating complex trees with multiple lines per leaf.

use treelog::Tree;

fn main() {
    // Create a complex tree structure similar to the original example
    let l1 = Tree::Leaf(vec![
        "line1".to_string(),
        "line2".to_string(),
        "line3".to_string(),
        "line4".to_string(),
    ]);
    let l2 = Tree::Leaf(vec!["only one line".to_string()]);
    
    let n1 = Tree::Node("node 1".to_string(), vec![l1.clone(), l2.clone()]);
    let n2 = Tree::Node("node 2".to_string(), vec![l2.clone(), l1.clone(), l2.clone()]);
    let n3 = Tree::Node("node 3".to_string(), vec![n1.clone(), l1.clone(), l2.clone()]);
    let n4 = Tree::Node("node 4".to_string(), vec![n1, n2, n3]);

    println!("Complex Tree with Multiple Lines:");
    println!("{}", n4.render_to_string());
}

