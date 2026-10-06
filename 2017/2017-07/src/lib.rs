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
        if let Some(matched) = regex.captures(line)
            && let Some(node) = matched.get(1)
            && let node = node.as_str().to_string()
            && let Some(value) = matched.get(2)
            && let value = value.as_str().to_string()
            && let Ok(value) = value.parse::<u32>()
        {
            maps.insert(node.to_string(), value);
            if let Some(child) = matched.get(3)
                && let child = child
                    .as_str()
                    .split(",")
                    .map(|str| str.trim().to_string())
                    .collect::<Vec<String>>()
            {
                let node = if let Some(node) = ids.get(&node.to_string()) {
                    *node
                } else {
                    let id = graph.add_node(node.to_string());
                    ids.insert(node, id);
                    id
                };

                for c_str in child {
                    let target = if let Some(node) = ids.get(&c_str) {
                        *node
                    } else {
                        let id = graph.add_node(c_str.to_string());
                        ids.insert(c_str, id);
                        id
                    };
                    graph.add_edge(node, target, 1);
                }
            }
        }
    });

    (graph, maps)
}

pub fn get_root(graph: &Graph<String, u32>) -> String {
    let node = graph.node_indices();
    let root = node
        .into_iter()
        .filter(|n| {
            graph.edge_indices().all(|e| {
                let (_source, target) = graph.edge_endpoints(e).unwrap();
                &target != n
            })
        })
        .collect::<Vec<NodeIndex>>();

    graph.node_weight(root[0]).unwrap().to_string()
}
pub mod part1 {
    use crate::{get_root, parse};

    pub fn solve(file: String) -> String {
        let (graph, _nodes) = parse(file);
        get_root(&graph)
    }
}

pub mod part2 {
    use crate::{get_root, parse};
    use petgraph::{Graph, visit::EdgeRef};
    use std::collections::{HashMap, VecDeque};

    fn find_outlier(vals: &[(u32, String)]) -> Option<(u32, String, u32)> {
        if vals.len() < 3 {
            return None;
        }

        let default = if vals[0].0 == vals[1].0 || vals[0].0 == vals[2].0 {
            vals[0].0
        } else {
            vals[1].0
        };

        vals.iter()
            .map(|x| (x.0, x.1.to_string(), default))
            .find(|x| x.0 != default)
    }

    fn find_imbalance(
        graph: Graph<String, u32>,
        values: HashMap<String, u32>,
    ) -> Option<(String, u32)> {
        let root = get_root(&graph);
        let mut sum = HashMap::new();
        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        while let Some(node) = nodes.pop_front() {
            let id = graph.node_indices().find(|n| graph[*n] == node).unwrap();

            let edges = graph.edges(id);
            let targets = edges
                .map(|e| {
                    let id = e.target();
                    graph.node_weight(id).unwrap().to_string()
                })
                .collect::<Vec<String>>();

            if targets.is_empty() {
                sum.insert(node.to_string(), *values.get(&node).unwrap());
                continue;
            }

            if !targets.iter().all(|n| sum.contains_key(n)) {
                for target in targets {
                    nodes.push_back(target);
                }
                nodes.push_back(node);
                continue;
            }

            let vals = targets
                .iter()
                .map(|string| (*sum.get(string).unwrap(), string.to_string()))
                .collect::<Vec<(u32, String)>>();

            if let Some((diff_value, diff_node, default)) = find_outlier(&vals) {
                let change_value = diff_value as i32 - default as i32;
                let node_value = *values.get(&diff_node).unwrap() as i32;
                println!("{:?} {node_value} {change_value}  ", vals);
                return Some((diff_node, (node_value - change_value) as u32));
            }
            let branch_value =
                vals.into_iter().map(|(val, _str)| val).sum::<u32>() + values.get(&node).unwrap();
            sum.insert(node, branch_value);
        }
        None
    }

    pub fn solve(file: String) -> Option<(String, u32)> {
        let (graph, values) = parse(file);
        find_imbalance(graph, values)
    }
}
