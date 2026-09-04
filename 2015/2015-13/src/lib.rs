
use std::collections::HashMap;

use petgraph::graph::DiGraph;
use regex::Regex;

pub fn parse(file: String) -> DiGraph<String, i64> {
    let mut ids = HashMap::new();
    let mut graph = DiGraph::new();

    let regex = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).").unwrap();
    file.lines().filter(|line| !line.is_empty())
    .for_each(|line| {
        if let Some(cap) = regex.captures(line)
        && let Some(w1) = cap.get(1)
        && let Some(sign) = cap.get(2)
        && let Some(value) = cap.get(3)
        && let Ok(n) = value.as_str().parse::<i64>()
        && let Some(w2) = cap.get(4)
        {
            let id1 = if let Some(id) = ids.get(&w1.as_str().to_string()) {
                *id
            } else {
                let id = graph.add_node(w1.as_str().to_string());
                ids.insert(w1.as_str().to_string(), id);
                id
            };
            let id2 = if let Some(id) = ids.get(&w2.as_str().to_string()) {
                *id
            } else {
                let id = graph.add_node(w2.as_str().to_string());
                ids.insert(w2.as_str().to_string(), id);
                id
            };
            if sign.as_str() == "gain" {
                graph.add_edge(id1, id2, n);
            } else if sign.as_str() == "lose" {
                graph.add_edge(id1, id2, -n);
            }
        }
    });

    graph
}

pub mod part1 {
    use std::collections::VecDeque;

use petgraph::graph::NodeIndex;
    use crate::parse;

    pub fn solve(file: String) -> i64 {
        let graph = parse(file);
        let mut positions: Vec<(i64, Vec<NodeIndex>)> = Vec::new();
        let ids = graph.node_indices().collect::<Vec<NodeIndex>>();
        let node_cnt= ids.len();
        let first = ids.first().unwrap();
        let mut temp: VecDeque<Vec<NodeIndex>> = VecDeque::new();
        temp.push_back(vec![*first]);
        while let Some(t) = temp.pop_back() {
            if t.len() == node_cnt {
                let mut pairs = t.windows(2)
                .map(|array| (array[0], array[1]))
                .collect::<Vec<(NodeIndex, NodeIndex)>>();
                pairs.push((*t.last().unwrap(),*t.first().unwrap()));

                let score = pairs.iter().map(|pair|
                 {
                    let e1 = graph.find_edge(pair.0, pair.1).unwrap();
                    let e2 = graph.find_edge(pair.1, pair.0).unwrap();
                    let w1 = graph.edge_weight(e1).unwrap();
                    let w2 = graph.edge_weight(e2).unwrap();
                    *w1 + *w2
                }).sum::<i64>();
                positions.push((score, t));
            } else {
                for id in ids.iter().clone() {
                    if ! t.contains(id) {
                        let mut new_t = t.clone();
                        new_t.push(*id);
                        temp.push_back(new_t);
                    }
                }
            }
        }

        positions.iter().map(|(score, _list)| *score)
        .max().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
