use std::collections::HashMap;

use petgraph::Graph;
use petgraph::prelude::NodeIndex;
use regex::Regex;

pub fn parse(file: String) -> (Graph<String, u32>, HashMap<String, u32>) {
    let mut graph = Graph::new();
    let mut maps = HashMap::new();
    let mut ids: HashMap<String, NodeIndex> = HashMap::new();
    let regex = Regex::new(r"(\w+) \((\d+)\)(?: -> (.*)){0,1}").unwrap();

    file.lines().for_each(|line| {
        if let Some(matched) = regex.captures(line) {
            if let Some(node) = matched.get(1)
            && let node = node.as_str().to_string()
            && let Some(value) = matched.get(2)
            && let value = value.as_str().to_string()
            && let Ok(value) = value.parse::<u32>()
            {
                maps.insert(node.to_string(), value);
                if let Some(child) = matched.get(3)
                && let child = child.as_str().split(",").map(|str| str.trim().to_string()).collect::<Vec<String>>()
                {
                    let node = if let Some(node) = ids.get(&node.to_string()) {
                        *node
                    } else {
                        let id = graph.add_node(node.to_string());
                        ids.insert( node, id);
                        id
                    };

                    for c_str in child {
                        let target = if let Some(node) = ids.get(&c_str) {
                            *node
                        } else {
                            let id = graph.add_node(c_str.to_string());
                            ids.insert( c_str, id);
                            id
                        };
                        graph.add_edge(node, target, 1);
                    }
                }
            }
        }
    });

    (graph, maps)
}

pub mod part1 {
    use petgraph::prelude::NodeIndex;
    use crate::parse;

    pub fn solve(file: String) -> String {

        let (graph, _nodes) = parse(file);

        let node = graph.node_indices();
        let root = node.into_iter().filter(|n| {
            // let edges = graph.edge_indices();
            graph.edge_indices().all(| e | {
                let (_source,target) = graph.edge_endpoints(e).unwrap();
                &target != n
            })
        }).collect::<Vec<NodeIndex>>();

        graph.node_weight(root[0]).unwrap().to_string()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
