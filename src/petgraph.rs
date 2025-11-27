//! Graph to/from tree conversion using petgraph.

use crate::tree::Tree;
use petgraph::visit::EdgeRef;

impl Tree {
    /// Converts a petgraph Graph to a Tree.
    ///
    /// Requires the `petgraph` feature.
    ///
    /// The root node is selected as the first node with no incoming edges,
    /// or the first node if all nodes have incoming edges (handles cycles).
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    /// use petgraph::Graph;
    ///
    /// let mut graph = Graph::<String, ()>::new();
    /// let a = graph.add_node("A".to_string());
    /// let b = graph.add_node("B".to_string());
    /// graph.add_edge(a, b, ());
    ///
    /// let tree = Tree::from_graph(&graph);
    /// ```
    #[cfg(feature = "petgraph")]
    pub fn from_graph<N, E, Ty, Ix>(graph: &petgraph::Graph<N, E, Ty, Ix>) -> Self
    where
        N: std::fmt::Display + Clone,
        E: Default,
        Ty: petgraph::EdgeType,
        Ix: petgraph::graph::IndexType,
    {
        if graph.node_count() == 0 {
            return Tree::new_node("empty".to_string());
        }

        // Find root: node with no incoming edges, or first node if all have incoming edges
        let root_idx = graph
            .node_indices()
            .find(|&idx| {
                graph
                    .edges_directed(idx, petgraph::Direction::Incoming)
                    .next()
                    .is_none()
            })
            .or_else(|| graph.node_indices().next())
            .expect("Graph has at least one node");

        Self::from_graph_recursive(graph, root_idx, &mut std::collections::HashSet::new())
    }

    #[cfg(feature = "petgraph")]
    fn from_graph_recursive<N, E, Ty, Ix>(
        graph: &petgraph::Graph<N, E, Ty, Ix>,
        node_idx: petgraph::graph::NodeIndex<Ix>,
        visited: &mut std::collections::HashSet<petgraph::graph::NodeIndex<Ix>>,
    ) -> Self
    where
        N: std::fmt::Display + Clone,
        E: Default,
        Ty: petgraph::EdgeType,
        Ix: petgraph::graph::IndexType,
    {
        // Handle cycles by checking if we've visited this node
        if visited.contains(&node_idx) {
            return Tree::new_leaf(format!("<cycle: {}>", graph[node_idx]));
        }
        visited.insert(node_idx);

        let node_data = graph[node_idx].clone();
        let label = format!("{}", node_data);

        // Get all outgoing edges
        let children: Vec<Tree> = graph
            .edges_directed(node_idx, petgraph::Direction::Outgoing)
            .map(|edge| {
                let target = edge.target();
                Self::from_graph_recursive(graph, target, visited)
            })
            .collect();

        visited.remove(&node_idx);

        if children.is_empty() {
            Tree::new_leaf(label)
        } else {
            Tree::Node(label, children)
        }
    }

    /// Converts a Tree to a petgraph Graph.
    ///
    /// Requires the `petgraph` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    /// use petgraph::Graph;
    ///
    /// let tree = Tree::Node("root".to_string(), vec![Tree::Leaf(vec!["item".to_string()])]);
    /// let graph: Graph<String, ()> = tree.to_graph();
    /// ```
    #[cfg(feature = "petgraph")]
    pub fn to_graph<N, E>(&self) -> petgraph::Graph<N, E>
    where
        N: From<String> + Clone,
        E: Default,
    {
        let mut graph = petgraph::Graph::<N, E>::new();
        let mut node_map = std::collections::HashMap::new();

        self.to_graph_recursive(&mut graph, &mut node_map, None);

        graph
    }

    #[cfg(feature = "petgraph")]
    fn to_graph_recursive<N, E>(
        &self,
        graph: &mut petgraph::Graph<N, E>,
        node_map: &mut std::collections::HashMap<*const Tree, petgraph::graph::NodeIndex>,
        parent: Option<petgraph::graph::NodeIndex>,
    ) where
        N: From<String> + Clone,
        E: Default,
    {
        let label = match self {
            Tree::Node(label, _) => label.clone(),
            Tree::Leaf(lines) => lines.first().cloned().unwrap_or_default(),
        };

        let node_idx = graph.add_node(N::from(label));
        node_map.insert(self as *const Tree, node_idx);

        if let Some(parent_idx) = parent {
            graph.add_edge(parent_idx, node_idx, E::default());
        }

        if let Tree::Node(_, children) = self {
            for child in children {
                child.to_graph_recursive(graph, node_map, Some(node_idx));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "petgraph")]
    #[test]
    fn test_tree_to_graph() {
        let tree = Tree::Node(
            "root".to_string(),
            vec![
                Tree::Leaf(vec!["item1".to_string()]),
                Tree::Node(
                    "sub".to_string(),
                    vec![Tree::Leaf(vec!["subitem".to_string()])],
                ),
            ],
        );
        let graph: petgraph::Graph<String, ()> = tree.to_graph();
        assert_eq!(graph.node_count(), 4); // root, item1, sub, subitem
    }

    #[cfg(feature = "petgraph")]
    #[test]
    fn test_graph_to_tree() {
        use petgraph::Graph;

        let mut graph = Graph::<String, ()>::new();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());
        graph.add_edge(a, b, ());
        graph.add_edge(a, c, ());

        let tree = Tree::from_graph(&graph);
        assert!(tree.is_node());
        assert_eq!(tree.label(), Some("A"));
    }
}
