use std::collections::{HashMap, HashSet};

use petgraph::graph::DiGraph;
use regex::Regex;

pub fn parse(file: String) -> (DiGraph<String, String>, HashSet<String>) {
    let regex = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();

    let list = file
        .lines()
        .map(|line| {
            let cap = regex.captures(line).unwrap();
            let src = cap.get(1).unwrap().as_str().to_string();
            let dst = cap.get(2).unwrap().as_str().to_string();
            (src, dst)
        })
        .collect::<Vec<(String, String)>>();

    let mut graph = DiGraph::new();
    let mut map = HashMap::new();

    for (src, dst) in list.iter() {
        map.entry(src.to_string()).or_insert_with(|| graph.add_node(src.to_string()));
        map.entry(dst.to_string()).or_insert_with(|| graph.add_node(dst.to_string()));
    }

    for (src, dst) in list.iter() {
        let src_node = map.get(src).unwrap();
        let dst_node = map.get(dst).unwrap();
        graph.add_edge(*src_node, *dst_node, format!("{}_{}", src, dst));
    }

    (
        graph,
        map.keys().map(|key| key.to_string())
            .collect::<HashSet<String>>(),
    )
}

pub mod part1 {
    use std::collections::BTreeSet;
    use crate::parse;

    pub fn solve(file: String) -> String {
        let (graph, nodes) = parse(file);
        let mut left_nodes = nodes.clone().into_iter().collect::<BTreeSet<String>>();
        let mut finish: Vec<String> = vec![];

        let edges = graph
            .edge_weights()
            .map(|str| {
                let (src, dst) = str.split_once("_").unwrap();
                (src.to_string(), dst.to_string())
            })
            .collect::<Vec<(String, String)>>();

        while ! left_nodes.is_empty() {
            let temp = left_nodes.clone();
            let node = temp.iter().find(|node| {
                let edge = edges.iter()
                    .filter(|(_src,dst)| dst == *node)
                    .map(|(src,dst)| (src.to_string(),dst.to_string()))
                    .collect::<Vec<(String,String)>>();

                // without require node
                edge.is_empty()
                ||
                // check require node already in finish
                edge.iter().all(|(src, _dst)| {
                    finish.contains(src)
                })

            }).unwrap();

            left_nodes.remove(node);
            finish.push(node.to_string());
        }

        finish.into_iter().collect::<String>()
    }
}

pub mod part2 {
    pub fn solve(_file: String, _:usize) -> u64 {
        0
    }
}
