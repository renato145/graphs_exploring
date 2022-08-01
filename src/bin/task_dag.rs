//! Example of transitive reduction on a graph of tasks
use graphs_exploring::{get_graph_viz, print_graph_dots};
use petgraph::prelude::*;
use petgraph::{
    algo::{
        toposort,
        tred::{dag_to_toposorted_adjacency_list, dag_transitive_reduction_closure},
    },
    graph::NodeIndex,
    visit::IntoEdgeReferences,
    Graph,
};

fn main() {
    let mut graph = Graph::<_, _>::new();
    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");
    graph.add_node("E");
    graph.extend_with_edges(&[
        (0, 1, 1),
        (0, 2, 1),
        (0, 3, 3),
        (0, 4, 2),
        (1, 3, 1),
        (2, 3, 1),
        (2, 4, 1),
        (3, 4, 2),
    ]);

    // Transitive reduction
    let sorted = toposort(&graph, None).expect("Not acyclic graph");
    let reduction = {
        let (res, _revmap) = dag_to_toposorted_adjacency_list::<_, NodeIndex>(&graph, &sorted);
        let (res, _revmap) = dag_transitive_reduction_closure(&res);
        res
    };
    let edges = reduction
        .edge_references()
        .map(|x| (x.source(), x.target()))
        .collect::<Vec<_>>();
    let g = Graph::<(), ()>::from_edges(edges);

    let dots = [
        get_graph_viz(&graph),
        get_graph_viz(&reduction),
        get_graph_viz(&g),
    ];
    print_graph_dots(&dots);
}
