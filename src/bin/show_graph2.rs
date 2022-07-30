use graphs_exploring::print_graph_svg;
use petgraph::Graph;

fn main() {
    let mut graph = Graph::<&str, &str>::new();
    let pg = graph.add_node("petgraph");
    let fb = graph.add_node("fixedbitset");
    let qc = graph.add_node("quickcheck");
    let rand = graph.add_node("rand");
    let libc = graph.add_node("libc");
    graph.extend_with_edges(&[
        (pg, fb, "requires"),
        (pg, qc, "requires"),
        (qc, rand, "needs"),
        (rand, libc, "needs"),
        (qc, libc, "also needs"),
    ]);
    print_graph_svg(&graph);
}
