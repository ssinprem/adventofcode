use petgraph::graph::*;

pub fn parse(file: String) -> UnGraph<String, ()> {
    let mut graph: UnGraph<String, ()> = UnGraph::new_undirected();

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if let Some((start, str_nodes)) = line.split_once(":") {
                let start: String = start.trim().to_string();

                let start_num = if let Some(id) = graph.node_indices().find(|i| graph[*i] == start)
                {
                    id
                } else {
                    graph.add_node(start)
                };

                for node in str_nodes.split_whitespace() {
                    let node = node.to_string();
                    let node_num =
                        if let Some(id) = graph.node_indices().find(|i| graph[*i] == node) {
                            id
                        } else {
                            graph.add_node(node)
                        };

                    graph.add_edge(start_num, node_num, ());
                }
            }
        });

    graph
}

pub mod part1 {
    use std::collections::HashSet;

use crate::parse;
    use petgraph::{algo::kosaraju_scc, visit::EdgeRef};
    use petgraph::graph::*;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    pub fn remove_3_edge(graph: &UnGraph<String, ()>) -> Option<UnGraph<String, ()>> {
        let edges: Vec<EdgeIndex> = graph.edge_indices().collect();
        let n = edges.len();
        let start_node = NodeIndex::new(0);
        let total_node = graph.node_count();
        (0..n ).into_par_iter().find_map_any(|i| {
            for j in i+1..n {
                for k in j+1..n {
                    if count_reachable_nodes(graph, start_node, vec![edges[k],edges[j],edges[i]]) < total_node {
                        let mut test = graph.clone();

                        test.remove_edge(edges[k]);
                        test.remove_edge(edges[j]);
                        test.remove_edge(edges[i]);

                        return Some(test);
                    }
                }
            }
            None
        })
    }

    pub fn count_reachable_nodes(
        graph: &UnGraph<String, ()>,
        start: NodeIndex,
        skip_edge: Vec<EdgeIndex>,
    ) -> usize {
        let mut visited = vec![false; graph.node_count()];
        let mut queue = Vec::new();
        
        visited[start.index()] = true;
        queue.push(start);
        let mut count = 1;

        while let Some(node) = queue.pop() {
            for edge_ref in graph.edges(node) {
                let edge_id = edge_ref.id();

                if skip_edge.contains(&edge_id) {
                    continue;
                }

                let neighbor = edge_ref.target();
                let nid = neighbor.index();
                if !visited[nid] {
                    visited[nid] = true;
                    queue.push(neighbor);
                    count += 1;
                }
            }
        }

        count
    }

    pub fn solve(file: String) -> u64 {
        let graph = parse(file);
        if let Some(new_graph) = remove_3_edge(&graph) {
            let group = kosaraju_scc(&new_graph);

            group
                .iter()
                .inspect(|g| println!("[{}] {:?}", g.len(), g))
                .map(|hs| hs.len())
                .product::<usize>() as u64
        } else {
            0
        }
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
