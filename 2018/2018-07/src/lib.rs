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
        if !map.contains_key(&src.to_string()) {
            map.insert(src.to_string(), graph.add_node(src.to_string()));
        }
        if !map.contains_key(&dst.to_string()) {
            map.insert(dst.to_string(), graph.add_node(dst.to_string()));
        }
    }

    for (src, dst) in list.iter() {
        let src_node = map.get(src).unwrap();
        let dst_node = map.get(dst).unwrap();
        graph.add_edge(*src_node, *dst_node, format!("{}_{}", src, dst));
    }

    (
        graph,
        map.iter()
            .map(|(key, _)| key.to_string())
            .collect::<HashSet<String>>(),
    )
}

pub mod part1 {
    use crate::parse;

    pub fn span_node(start: String, edges: &Vec<(String, String)>) -> Vec<String> {
        let mut temp = vec![start.to_string()];

        let mut nexts = edges
            .iter().filter(|(src, dst)| src == &start)
            .map(|(_,dst)| dst.to_string())
            .collect::<Vec<_>>();
        nexts.sort();

        println!("=> {nexts:?}");
        for next in nexts {
            if edges.iter().filter(|(s,d)| {
                d == &next
            }).count() == 1 {
                temp = [temp, span_node(next, edges)].concat();
            }
        }

        temp
    }

    pub fn solve(file: String) -> String {
        let (graph, nodes) = parse(file);
        let mut left_nodes = nodes.clone();
        let mut finish: Vec<String> = vec![];

        let edges = graph
            .edge_weights()
            .map(|str| {
                let (src, dst) = str.split_once("_").unwrap();
                (src.to_string(), dst.to_string())
            })
            .collect::<Vec<(String, String)>>();

        // find first node
        let mut leftest = left_nodes
            .iter()
            .filter(|n_str| !edges.iter().any(|(_src, dst)| *dst == **n_str))
            .collect::<Vec<_>>();
        leftest.sort();

        println!("{leftest:?}");

        for l in leftest {
            let res = span_node(l.to_string(), &edges);
            println!("{l} -> {:?}", res);
            finish = [finish, res].concat();
        }

        finish.into_iter().collect::<String>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
