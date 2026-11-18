use std::collections::HashMap;

use petgraph::graph::UnGraph;

pub fn parse(file: String) -> UnGraph<String, usize> {
    let mut map = HashMap::new();
    let mut graph = UnGraph::new_undirected();
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
                res
                    .iter()
                    .find_map(|(k, v)| if k == n { Some(*v as usize) } else { None })
            })
            .sum::<usize>()
    }
}

pub mod part2 {
    use crate::parse;
    use petgraph::algo::dijkstra;

    pub fn solve(file: String) -> usize {
        let graph = parse(file);

        let nodes = graph.node_indices().collect::<Vec<_>>();

        let you_nodes = nodes
            .iter()
            .find(|node_id| graph.node_weight(**node_id) == Some(&"YOU".to_string()))
            .unwrap();
        let san_nodes = nodes
            .iter()
            .find(|node_id| graph.node_weight(**node_id) == Some(&"SAN".to_string()))
            .unwrap();

        let you_parent = graph.neighbors_undirected(*you_nodes).collect::<Vec<_>>()[0];
        let san_parent = graph.neighbors_undirected(*san_nodes).collect::<Vec<_>>()[0];

        let res = dijkstra(&graph, you_parent, Some(san_parent), |_| 1);
        res.iter()
            .find_map(|(k, v)| {
                if k == &san_parent {
                    Some(*v as usize)
                } else {
                    None
                }
            })
            .unwrap()
    }
}
