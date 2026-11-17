use std::collections::HashMap;

use petgraph::Graph;

pub fn parse(file: String) -> Graph<String, usize> {
    let mut map = HashMap::new();
    let mut graph = Graph::new();
    file.lines().for_each(|line| {
        let (src, dst) = line.split_once(")").unwrap();
        let src = src.to_string();
        let dst = dst.to_string();

        let src_id = if let Some(id) = map.get(&src) {
            *id
        } else {
            let id = graph.add_node(src.to_string());
            map.insert(src.to_string(), id);
            id
        };
        let dst_id = if let Some(id) = map.get(&dst) {
            *id
        } else {
            let id = graph.add_node(dst.to_string());
            map.insert(dst.to_string(), id);
            id
        };
        graph.add_edge(src_id, dst_id, 1);
    });

    graph
}

pub mod part1 {
    use crate::parse;
    use petgraph::algo::dijkstra;

    pub fn solve(file: String) -> usize {
        let graph = parse(file);

        let nodes = graph.node_indices().collect::<Vec<_>>();

        let com_nodes = nodes
            .iter()
            .find(|node_id| graph.node_weight(**node_id) == Some(&"COM".to_string()))
            .unwrap();

        nodes
            .iter()
            .filter_map(|n| {
                let res = dijkstra(&graph, *com_nodes, Some(*n), |_| 1);
                let len = res
                    .iter()
                    .find_map(|(k, v)| if k == n { Some(*v as usize) } else { None });
                len
            })
            .sum::<usize>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
