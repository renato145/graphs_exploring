use petgraph::{
    dot::Dot,
    visit::{GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable},
};
use std::{collections::HashMap, fmt, fmt::Write};

pub fn get_graph_viz<G>(graph: G) -> String
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: fmt::Debug,
    G::NodeWeight: fmt::Debug,
{
    let dot = Dot::new(graph);
    format!("{:?}", dot)
}

pub fn print_graph_svg<G>(graph: G)
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: fmt::Debug,
    G::NodeWeight: fmt::Debug,
{
    println!("{}", get_graph_viz(graph));
}

pub fn print_graph_dots(dots: &[String]) {
    println!("digraph {{");
    dots.iter().enumerate().for_each(|(i, dot)| {
        let mut dot = dot.to_string().replacen("digraph", "subgraph", 1);
        add_suffix(&mut dot, &format!("{i:03}"));
        println!("{}", dot);
    });
    println!("}}");
}

pub fn print_graphs_svg<G>(graphs: &[G])
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + GraphProp,
    G::EdgeWeight: fmt::Debug,
    G::NodeWeight: fmt::Debug,
{
    println!("digraph {{");
    let dots = graphs.iter().map(get_graph_viz).collect::<Vec<_>>();
    print_graph_dots(&dots[..]);
    println!("}}");
}

fn add_suffix(dot: &mut String, suffix: &str) {
    let mut map = HashMap::new();
    for line in dot.lines().skip(1) {
        if line.contains("->") {
            break;
        }
        let (id, _) = line
            .trim_start()
            .split_once(" [")
            .unwrap_or_else(|| panic!("Error on line: {line}"));

        map.insert(id, format!("{id}{suffix}"));
    }

    let mut lines = dot.lines();
    let mut res = lines.next().unwrap().to_string();
    for line in lines {
        res.push('\n');
        if line == "}" {
            res.push_str(line);
        } else {
            let (a, rest) = line
                .split_once('[')
                .unwrap_or_else(|| panic!("Error on line: {line}"));
            let (mut a, mut b) = a
                .split_once("->")
                .map(|x| (x.0.trim().to_string(), Some(x.1.trim().to_string())))
                .unwrap_or_else(|| (a.trim().to_string(), None));
            let mut a_changed = false;
            let mut b_changed = b.is_none();
            for (k, v) in map.iter() {
                if !a_changed && &a == k {
                    a = v.to_string();
                    a_changed = true;
                }
                if let Some(b) = b.as_mut() {
                    if !b_changed && b == k {
                        *b = v.to_string();
                        b_changed = true;
                    }
                }
            }
            write!(res, "{a}").unwrap();
            if let Some(b) = b {
                write!(res, " -> {b}").unwrap();
            }
            write!(res, " [{rest}").unwrap();
        }
    }
    *dot = res;
}
