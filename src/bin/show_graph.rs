use graphs_exploring::print_graph_svg;
use petgraph::Graph;

fn main() {
    let mut graph = Graph::<_, ()>::new();
    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");
    graph.extend_with_edges(&[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    print_graph_svg(&graph);
}
