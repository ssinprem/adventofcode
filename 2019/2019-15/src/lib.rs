use std::collections::HashMap;

use aoc_2019_11::Dir::{self, *};
use petgraph::graph::{NodeIndex, UnGraph};

use crate::Cell::*;

#[derive(PartialEq)]
pub enum Cell {
    Space,
    Wall,
    Oxygen,
}

pub fn dir2n(dir: &Dir) -> i128 {
    match dir {
        N => 1,
        S => 2,
        W => 3,
        E => 4,
    }
}

pub fn dir_opposite(dir: &Dir) -> Dir {
    match dir {
        N => S,
        S => N,
        W => E,
        E => W,
    }
}

pub fn next_pos(curr_pos: (isize, isize), dir: &Dir) -> (isize, isize) {
    match dir {
        N => (curr_pos.0, curr_pos.1 - 1),
        S => (curr_pos.0, curr_pos.1 + 1),
        W => (curr_pos.0 - 1, curr_pos.1),
        E => (curr_pos.0 + 1, curr_pos.1),
    }
}

pub fn _display(pad: &HashMap<(isize, isize), Cell>, curr: &(isize, isize)) {
    let mut min_x = pad.keys().map(|(x, _y)| *x).min().unwrap();
    let mut max_x = pad.keys().map(|(x, _y)| *x).max().unwrap();
    let mut min_y = pad.keys().map(|(_x, y)| *y).min().unwrap();
    let mut max_y = pad.keys().map(|(_x, y)| *y).max().unwrap();

    min_x = min_x.min(curr.0);
    max_x = max_x.max(curr.0);
    min_y = min_y.min(curr.1);
    max_y = max_y.max(curr.1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if curr == &(x, y) {
                print!("D");
            } else if let Some(v) = pad.get(&(x, y)) {
                print!(
                    "{}",
                    match *v {
                        Space => ".",
                        Wall => "#",
                        Oxygen => "O",
                    }
                );
            } else {
                print!("?");
            }
        }
        println!();
    }
}

pub fn generate_map(memory: HashMap<usize, i128>) -> HashMap<(isize, isize), Cell> {
    let mut memory = memory.clone();
    let mut pc = 0;
    let mut relative_base = 0;
    let mut curr_pos = (0, 0);
    let mut path = vec![];
    let mut map = HashMap::new();
    map.insert(curr_pos, Space);

    loop {
        let (dir, backtrack) = if let Some(d) = [N, S, W, E]
            .iter()
            .find(|d| !map.contains_key(&next_pos(curr_pos, d)))
        {
            (*d, false)
        } else if let Some(last) = path.pop() {
            (dir_opposite(&last), true)
        } else {
            break;
        };

        let Some(out) = aoc_2019_11::process(&mut memory, &mut pc, &mut relative_base, dir2n(&dir))
        else {
            break;
        };
        let next = next_pos(curr_pos, &dir);
        match out {
            0 => {
                map.insert(next, Wall);
            }
            1 | 2 => {
                map.insert(next, if out == 1 { Space } else { Oxygen });
                curr_pos = next;
                if !backtrack {
                    path.push(dir);
                }
            }
            _ => unreachable!(),
        }
    }
    map
}

#[allow(clippy::type_complexity)]
pub fn get_graph(
    map: &HashMap<(isize, isize), Cell>,
) -> (
    UnGraph<(isize, isize), usize>,
    HashMap<(isize, isize), NodeIndex>,
) {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = HashMap::new();

    let min_x = map.keys().min_by_key(|p| p.0).unwrap().0;
    let max_x = map.keys().max_by_key(|p| p.0).unwrap().0;
    let min_y = map.keys().min_by_key(|p| p.1).unwrap().1;
    let max_y = map.keys().max_by_key(|p| p.1).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(cell) = map.get(&(x, y))
                && cell != &Wall
            {
                let node = if let Some(n) = nodes.get(&(x, y)) {
                    *n
                } else {
                    let n = graph.add_node((x, y));
                    nodes.insert((x, y), n);
                    n
                };

                [N, E, W, S].iter().for_each(|d| {
                    let next_pos = next_pos((x, y), d);
                    if let Some(next) = map.get(&next_pos)
                        && next != &Wall
                    {
                        let n_node = if let Some(n) = nodes.get(&next_pos) {
                            *n
                        } else {
                            let n = graph.add_node((x, y));
                            nodes.insert(next_pos, n);
                            n
                        };
                        if !graph.contains_edge(n_node, node) {
                            graph.add_edge(node, n_node, 1);
                        }
                    }
                });
            }
        }
    }
    (graph, nodes)
}

pub mod part1 {
    use petgraph::algo::dijkstra;
    use std::collections::HashMap;

    use crate::{
        _display,
        Cell::{self, Oxygen},
        generate_map, get_graph,
    };

    pub fn get_few_move(map: &HashMap<(isize, isize), Cell>) -> usize {
        let (graph, nodes) = get_graph(map);

        let start = (0, 0);
        let end = *map.iter().find(|(_, c)| c == &&Oxygen).unwrap().0;

        let start_node = nodes[&start];
        let oxygen_node = nodes[&end];
        let res = dijkstra(&graph, start_node, None, |_| 1);
        res[&oxygen_node]
    }

    pub fn solve(file: String) -> usize {
        let memory = aoc_2019_09::parse(file);
        let map = generate_map(memory);
        _display(&map, &(0, 0));
        get_few_move(&map)
    }
}

pub mod part2 {
    use petgraph::algo::dijkstra;
    use std::collections::HashMap;

    use crate::{
        Cell::{self, Oxygen},
        generate_map, get_graph,
    };

    pub fn get_far_move(map: &HashMap<(isize, isize), Cell>) -> usize {
        let (graph, nodes) = get_graph(map);

        let end = map.iter().find(|(_, c)| c == &&Oxygen).unwrap().0;

        let oxygen_node = nodes[end];
        let res = dijkstra(&graph, oxygen_node, None, |_| 1);
        *res.values().max().unwrap()
    }

    pub fn solve(file: String) -> usize {
        let memory = aoc_2019_09::parse(file);
        let map = generate_map(memory);
        get_far_move(&map)
    }
}
