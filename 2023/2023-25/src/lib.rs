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
    use std::cmp::Reverse;
    use std::collections::{HashMap, VecDeque};
    use crate::parse;
    use petgraph::{algo::kosaraju_scc, visit::EdgeRef};
    use petgraph::graph::*;
    use rand::seq::IndexedRandom;

    pub fn remove_3_edge(graph: &UnGraph<String, ()>) -> Option<UnGraph<String, ()>> {
        let nodes : Vec<NodeIndex> = graph.node_indices().collect();
        let mut edge_count = HashMap::new();
        let mut rng = rand::rng();

        for _ in 0..300 {
            if let (Some(start), Some(end)) = (nodes.choose(&mut rng), nodes.choose(&mut rng)) {
                if start == end { continue; }

                if let Some(path_edges) = find_path(graph, *start, *end) {
                    for edge in path_edges {
                        *edge_count.entry(edge).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut edge_list: Vec<_> = edge_count.iter().collect();
        edge_list.sort_by_key(|(_e,v)| Reverse(*v));

        let mut targets = [
            *edge_list[0].0,
            *edge_list[1].0,
            *edge_list[2].0,
        ];
        targets.sort();

        let mut new_graph = graph.clone();
        new_graph.remove_edge(targets[0]);
        new_graph.remove_edge(targets[1]);
        new_graph.remove_edge(targets[2]);

        Some(new_graph)
    }

    pub fn find_path(
        graph: &UnGraph<String, ()>,
        start: NodeIndex,
        end: NodeIndex,
    ) -> Option<Vec<EdgeIndex>> {
        let mut visited = vec![false; graph.node_count()];
        let mut parent = HashMap::new();
        let mut queue = VecDeque::new();
        
        visited[start.index()] = true;
        queue.push_back(start);
        
        let mut found = false;
        while let Some(node) = queue.pop_front() {
            if node == end {
                found = true;
                break;
            }
            for edge_ref in graph.edges(node) {
                let edge_id = edge_ref.id();

                let neighbor = edge_ref.target();
                let nid = neighbor.index();
                if !visited[nid] {
                    visited[nid] = true;
                    queue.push_back(neighbor);
                    parent.insert(neighbor, (edge_id, node));
                }
            }
        }

        if !found { return None; }

        let mut path_edges = Vec::new();
        let mut curr = end;
        while curr != start {
            if let Some((edge_id, prev_node)) = parent.get(&curr) {
                path_edges.push(*edge_id);
                curr = *prev_node
            } else {
                break;
            }
        }
        Some(path_edges)
    }

    pub fn solve(file: String) -> u64 {
        let graph = parse(file);
        if let Some(new_graph) = remove_3_edge(&graph) {
            let group = kosaraju_scc(&new_graph);

            group
                .iter()
                .inspect(|g| println!("[{}] ", g.len()))
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
