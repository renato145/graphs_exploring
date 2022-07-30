use petgraph::{
    dot::Dot,
    visit::{GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable},
};
use std::fmt;

pub fn print_graph_svg<G>(graph: G)
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: fmt::Debug,
    G::NodeWeight: fmt::Debug,
{
    let dot = Dot::new(graph);
    println!("{:?}", dot);
}
