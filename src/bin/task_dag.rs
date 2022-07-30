//! Example of transitive reduction on a graph of tasks
use petgraph::{dot::Dot, Graph};

fn main() {
    let mut graph = Graph::<_, ()>::new();
    graph.add_node("A"); // 0
    graph.add_node("B"); // 1
    graph.add_node("C"); // 2
    graph.add_node("D"); // 3
    graph.add_node("E"); // 4
    graph.extend_with_edges(&[
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (1, 3),
        (2, 3),
        (2, 4),
        (3, 4),
    ]);

    let dot = format!("{:?}", Dot::new(&graph)).replacen("digraph", "subgraph", 1);
    println!("{}", dot);
}
