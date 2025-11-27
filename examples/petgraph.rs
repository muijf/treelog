//! Example: Converting between trees and petgraph graphs.

use treelog::Tree;

fn main() {
    #[cfg(feature = "petgraph")]
    {
        use petgraph::Graph;

        println!("=== Tree to Graph ===");
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

        let graph: Graph<String, ()> = tree.to_graph();
        println!("Converted to graph:");
        println!("  Nodes: {}", graph.node_count());
        println!("  Edges: {}", graph.edge_count());
        println!();

        println!("=== Graph to Tree ===");
        let mut graph = Graph::<String, ()>::new();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());
        let d = graph.add_node("D".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(a, c, ());
        graph.add_edge(b, d, ());

        println!("Graph structure:");
        println!("  A -> B");
        println!("  A -> C");
        println!("  B -> D");
        println!();

        let tree = Tree::from_graph(&graph);
        println!("Converted to tree:");
        println!("{}", tree.render_to_string());
    }

    #[cfg(not(feature = "petgraph"))]
    {
        println!("Note: Enable 'petgraph' feature to use graph conversion.");
        println!("Example usage:");
        println!("  let graph: Graph<String, ()> = tree.to_graph();");
        println!("  let tree = Tree::from_graph(&graph);");
    }
}
