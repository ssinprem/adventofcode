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
    use crate::parse;
    use petgraph::algo::kosaraju_scc;
    use petgraph::graph::*;

    pub fn remove_3_edge(graph: &UnGraph<String, ()>) -> Option<UnGraph<String, ()>> {
        let edges: Vec<EdgeIndex> = graph.edge_indices().collect();
        let n = edges.len();

        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let mut test = graph.clone();

                    test.remove_edge(edges[i]);
                    test.remove_edge(edges[j]);
                    test.remove_edge(edges[k]);

                    let groups = kosaraju_scc(&test);
                    println!("{i} {j} {k}  {}",groups.len());
                    if groups.len() == 2 {
                        return Some(test.clone());
                    }
                }
            }
        }
        None
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
