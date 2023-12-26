use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use rustworkx_core::Result;
use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::graph::UnGraph};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

struct Problem25 {
    size: usize,
    edges: Vec<(usize, usize)>,
}

impl Problem for Problem25 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2)
    where
        F1: FnOnce(&dyn Display),
        F2: FnOnce(&dyn Display),
    {
        let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
        let vertices = (0..self.size).map(|_| graph.add_node(())).collect_vec();

        self.edges.iter().for_each(|(u, v)| {
            let (u, v) = (vertices[*u], vertices[*v]);
            graph.extend_with_edges(&vec![(u, v), (v, u)]);
        });

        let min_cut: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));

        let s = min_cut.unwrap().unwrap().1.len();

        report_first(&(s * (self.size - s)));

        report_second(&"All done!");
    }

    fn parse(lines: Vec<String>) -> Self {
        let edges = lines
            .into_iter()
            .flat_map(|line| {
                let (from, to_adj) = line.split(": ").collect_tuple().unwrap();

                to_adj
                    .split(' ')
                    .map(|to| (from.to_string(), to.to_string()))
                    .collect_vec()
            })
            .collect_vec();

        let name_mapping = edges
            .clone()
            .into_iter()
            .flat_map(|(u, v)| vec![u, v])
            .collect::<HashSet<String>>()
            .into_iter()
            .enumerate()
            .map(|(idx, name)| (name, idx))
            .collect::<HashMap<_, _>>();

        Self {
            size: name_mapping.len(),
            edges: edges
                .into_iter()
                .map(|(u, v)| (name_mapping[&u], name_mapping[&v]))
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem25>("inputs/25.txt");
}
