use std::collections::HashMap;
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;
use petgraph::prelude::NodeIndex;
use regex::Regex;

pub fn parse(file: String) -> (UnGraph<String, u32>, HashMap<String, NodeIndex>) {
    let mut graph : UnGraph<String, u32> = UnGraph::<String, u32>::new_undirected();
    let mut id: HashMap<String, NodeIndex> = HashMap::new();
    let regex = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();

    file.lines().for_each(|line| {
        if let Some(cap) = regex.captures(line)
        && let Some(w1) = cap.get(1)
        && let Some(w2) = cap.get(2)
        && let Some(v) = cap.get(3)
        {
            let id1 = if let Some(id) = id.get(w1.as_str()) {
                *id
            } else {
                let id1 = graph.add_node(w1.as_str().to_string());
                id.insert(w1.as_str().to_string(), id1);
                id1
            };
            let id2 = if let Some(id) = id.get(w2.as_str()) {
                *id
            } else {
                let id2 = graph.add_node(w2.as_str().to_string());
                id.insert(w2.as_str().to_string(), id2);
                id2
            };
            graph.add_edge(id1, id2, v.as_str().parse::<u32>().unwrap());
        }
    });

    (graph, id)
}

pub fn pathing_score(graph: UnGraph<String, u32>, node_ids: HashMap<String, NodeIndex>)
-> Vec<(u32, Vec<NodeIndex>)>
{
    let mut working_paths: Vec<(Vec<NodeIndex>, u32)> = node_ids.iter().map(|node| (vec![*node.1], 0)).collect();
    let node_count = graph.node_count();
    let mut scores = Vec::<(u32, Vec<NodeIndex>)>::new();
    while let Some((path, score)) = working_paths.pop() {
        if path.len() == node_count {
            scores.push((score, path));
            continue;
        }
        let last = path.last().unwrap();
        let edges = graph.neighbors_undirected(*last);

        for edge in edges {
            if path.contains(&edge) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(edge);
            let edge_score = graph.edges(*last).find(|e| e.source() == edge || e.target() == edge).unwrap();
            let edge_score = edge_score.weight();
            let new_score = score + *edge_score;
            working_paths.push((new_path, new_score));
        }
    }
    scores
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let (graph, node_ids) = parse(file);
        let paths = pathing_score(graph, node_ids);
        paths.into_iter().min_by_key(|(score,_path)| *score).unwrap().0 as u64
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let (graph, node_ids) = parse(file);
        let paths = pathing_score(graph, node_ids);
        paths.into_iter().max_by_key(|(score,_path)| *score).unwrap().0 as u64
    }
}
