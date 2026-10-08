use std::collections::{HashMap, HashSet};

use petgraph::{graph::{NodeIndex, UnGraph}, visit::Dfs};
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

pub fn grouping(graph: UnGraph<u32, ()>) -> HashMap<usize, HashSet<u32>> {
    let mut nodes: HashMap<usize, HashSet<u32>> = HashMap::new();
    let mut group: usize = 0;
    while let Some(first_node) = graph
        .node_indices()
        .find(|node| 
            !nodes.iter().any(|(_grp_id, grp)| 
                grp.iter().any(|n| {
                    graph.node_weight(*node) == Some(n)
                })
            )
        )
    {
        let mut dfs = Dfs::new(&graph, first_node);
        while let Some(n) = dfs.next(&graph) {
            let value = *graph.node_weight(n).expect("cannot get value");
            nodes.entry(group)
            .and_modify(|node_group| {
                node_group.insert(value);
            })
            .or_insert(HashSet::from([value]));
        }
        group += 1;
    }
    nodes
}
pub mod part1 {
    use crate::{grouping, parse};

    pub fn solve(file: String) -> usize {
        let graph = parse(file);
        let nodes = grouping(graph);

        if let Some((_grp_id, first_group)) = nodes.iter()
            .find(|(_grp_id, grp_items)| grp_items.contains(&0))
        {
            println!("{first_group:?}");
            first_group.len()
        } else {
            0
        }
    }
}

pub mod part2 {
    use crate::{grouping, parse};

    pub fn solve(file: String) -> usize {
        let graph = parse(file);
        let nodes = grouping(graph);
        println!("{nodes:?}");
        nodes.len()
    }
}
