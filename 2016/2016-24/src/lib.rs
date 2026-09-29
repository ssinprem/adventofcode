use std::collections::HashMap;

use grid::Grid;
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::UnGraph;
use petgraph::prelude::NodeIndex;

use crate::Cell::{Num, Space, Wall};

#[derive(Default, Debug, PartialEq, Clone, Eq, Hash)]
pub enum Cell {
    #[default]
    None,
    Space,
    Wall,
    Num(u32),
}

pub fn parse(file: String) -> Grid<Cell> {
    let mut grid = Grid::new(0, 0);
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            grid.push_row(
                line.chars()
                    .map(|c| match c {
                        '.' => Cell::Space,
                        '#' => Cell::Wall,
                        c if let Some(n) = c.to_digit(10) => Cell::Num(n),
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        });
    grid
}

pub fn find_node_tarveller(grid: &Grid<Cell>) -> UnGraph<u32, usize> {
    let mut full_graph = UnGraph::<(usize, usize), usize>::new_undirected();
    let mut ids = HashMap::<(usize, usize), (NodeIndex, Cell)>::new();
    grid.iter_rows().enumerate().for_each(|(y, rows)| {
        rows.enumerate().for_each(|(x, cell)| {
            if *cell == Cell::Wall {
                return;
            }

            let curr_node = if let Some((id, _cell)) = ids.get(&(y, x)) {
                *id
            } else {
                let id = full_graph.add_node((y, x));
                ids.insert((y, x), (id, cell.clone()));
                id
            };

            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .for_each(|(dy, dx)| {
                    let next = ((dy + y as isize) as usize, (dx + x as isize) as usize);
                    if let Some(next_cell) = grid.get(next.0, next.1)
                        && next_cell != &Wall
                    {
                        let next_node = if let Some((id, _cell)) = ids.get(&(next.0, next.1)) {
                            *id
                        } else {
                            let id = full_graph.add_node((next.0, next.1));
                            ids.insert((next.0, next.1), (id, next_cell.clone()));
                            id
                        };
                        full_graph.add_edge(curr_node, next_node, 1);
                    }
                });
        });
    });

    let mut graph = UnGraph::<u32, usize>::new_undirected();
    let mut node_id = HashMap::new();
    let nodes = ids
        .values()
        .filter(|(_nodeid, cell)| matches!(cell, Cell::Num(_)))
        .collect::<Vec<_>>();
    for (_node, cell) in &nodes {
        if let Cell::Num(n) = cell {
            node_id.insert(*n, graph.add_node(*n));
        }
    }
    nodes.into_iter().permutations(2).for_each(|pair| {
        if let (node1, Cell::Num(n1)) = pair[0]
            && let (node2, Cell::Num(n2)) = pair[1]
        {
            let cost = dijkstra(&full_graph, *node1, Some(*node2), |_| 1);
            let new_node1 = node_id.get(n1).unwrap();
            let new_node2 = node_id.get(n2).unwrap();
            graph.add_edge(*new_node1, *new_node2, *cost.get(node2).unwrap());
        }
    });

    graph
}

pub fn _display(grid: &Grid<Cell>) -> String {
    let mut output = String::new();

    grid.iter_rows().for_each(|rows| {
        rows.into_iter().for_each(|cell| {
            if cell == &Wall {
                output += "#"
            } else if cell == &Space {
                output += "."
            } else if let Num(n) = cell {
                output += format!("{n}").as_str()
            }
        });
        output += "\n"
    });
    output
}
pub mod part1 {
    use crate::{_display, find_node_tarveller, parse};
    use itertools::Itertools;

    pub fn solve(file: String) -> usize {
        let grid = parse(file);
        // println!("{}", _display(&grid));
        let node_graph = find_node_tarveller(&grid);

        let nodes = node_graph.node_indices().collect::<Vec<_>>();
        let len = nodes.len();
        nodes
            .into_iter()
            .permutations(len)
            .filter(|path| node_graph.node_weight(path[0]) == Some(&0))
            .map(|path| {
                path.windows(2)
                    .map(|pairs| {
                        let p1 = pairs[0];
                        let p2 = pairs[1];

                        let edge = node_graph.find_edge(p1, p2).expect("cannot find edge");
                        node_graph.edge_weight(edge).unwrap()
                    })
                    .sum::<usize>()
            })
            .min()
            .unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
