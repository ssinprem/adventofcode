use std::collections::HashMap;

use petgraph::graph::{NodeIndex, UnGraph};
use regex::*;

pub fn get_node_id(
    graph: &mut UnGraph<u32, ()>,
    node_ids: &mut HashMap<u32, NodeIndex>,
    num: u32,
) -> NodeIndex {
    if let Some(id) = node_ids.get(&num) {
        *id
    } else {
        let id = graph.add_node(num);
        node_ids.insert(num, id);
        id
    }
}

pub fn parse(file: String) -> UnGraph<u32, ()> {
    let mut graph = UnGraph::new_undirected();
    let regex = Regex::new(r"(\d+) <-> (.*)").unwrap();
    let mut node_ids = HashMap::new();

    file.lines().for_each(|line| {
        if let Some(matched) = regex.captures(line)
            && let Some(num) = matched.get(1)
            && let Ok(num) = num.as_str().parse::<u32>()
            && let Some(ns) = matched.get(2)
            && let ns = ns
                .as_str()
                .split(",")
                .map(|str| str.trim().parse::<u32>().expect("cannot parse"))
                .collect::<Vec<u32>>()
        {
            let node = get_node_id(&mut graph, &mut node_ids, num);

            for n in ns {
                let target = get_node_id(&mut graph, &mut node_ids, n);
                graph.update_edge(node, target, ());
            }
        }
    });
    graph
}

pub mod part1 {
    use std::collections::HashSet;

    use crate::parse;
    use petgraph::visit::Dfs;

    pub fn solve(file: String) -> usize {
        let graph = parse(file);
        let mut nodes = HashSet::new();
        if let Some(first_node) = graph
            .node_indices()
            .find(|n| graph.node_weight(*n) == Some(&0))
        {
            let mut dfs = Dfs::new(&graph, first_node);

            while let Some(n) = dfs.next(&graph) {
                nodes.insert(n);
            }
        }

        nodes.len()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
