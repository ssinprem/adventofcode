use std::collections::{HashMap, HashSet};

use grid::Grid;
use petgraph::{algo::{astar}, graph::{NodeIndex, UnGraph}};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Cell {
    #[default]
    Space,
    Start,
    Wall,
    Key(u8),
    Door(u8),
}

pub fn parse(file: String) -> (Grid<Cell>, (usize,usize)) {
    let mut walker = (0,0);
    let mut map = Grid::new(0, 0);
    file.lines().enumerate().for_each(|(y,line)| {
        map.push_row(
            line.chars().enumerate().map(|(x,char)| {
                match char {
                    '@' => {
                        walker = (x,y);
                        Cell::Start
                    },
                    '.' => Cell::Space,
                    '#' => Cell::Wall,
                    c if c.is_lowercase() => Cell::Key(c as u8 - b'a'),
                    c if c.is_uppercase() => Cell::Door(c as u8 - b'A'),
                    _ => unreachable!()
                }
            }).collect()
        );
    });
    (map, walker)
}

#[allow(clippy::type_complexity)]
pub fn convert_map_to_graph(map: &Grid<Cell>)
 -> 
( UnGraph<((usize,usize),Cell), usize>, HashMap<(usize,usize), (NodeIndex, Cell)> ) {
    // big graph for every node
    let mut big_graph = UnGraph::new_undirected();
    let mut big_nodes = HashMap::new();
    
    // graph for only every node, not-wall and space
    let mut graph = UnGraph::new_undirected();
    let mut nodes= HashMap::new();

    let rows = map.rows();
    let cols = map.cols();
    
    // add node for every cell
    for row in 0..rows {
        for col in 0..cols {
            if let Some(cell) = map.get(row, col) {
                big_nodes.insert(
                    (col,row),
                    (
                        big_graph.add_node(((col,row), cell.clone())),
                        cell.clone()
                    )
                );

                nodes.insert(
                    (col,row),
                    (
                        graph.add_node( ((col,row),cell.clone())),
                        cell.clone()
                    )
                );
            }
        }
    }

    // add edge for not-wall cell
    for row in 0..rows {
        for col in 0..cols {
            if let Some((id, cel)) = big_nodes.get(&(col,row)) 
            && cel != &Cell::Wall
            {
                for (dx,dy) in [(-1,0),(1,0),(0,1),(0,-1)] {
                    let ncol = (col as isize + dx) as usize;
                    let nrow = (row as isize + dy) as usize;
                    if let Some((nid, ncel)) =  big_nodes.get(&(ncol,nrow)) 
                    && ncel != &Cell::Wall
                    && big_graph.edges_connecting(*id, *nid).count() == 0
                    {
                        big_graph.add_edge(*id, *nid, 1);
                    }
                }
            }
        }
    }

    for (pos,(node,cel)) in big_nodes.iter() {
        if matches!(cel, Cell::Start | Cell::Key(_) | Cell::Door(_))
        {
            // println!("{pos:?} {node:?} {cel:?}");
            let anode = nodes.get(&(pos.0,pos.1)).unwrap().0;

            let mut ignore = HashSet::new();
            let mut targets = HashMap::new();
            while let Some((cost, path)) = astar(&big_graph,
                *node,
                |n| {
                    n != *node && ! targets.contains_key(&n) && ! ignore.contains(&n) &&
                    matches!(big_graph.node_weight(n).unwrap().1, Cell::Key(_) | Cell::Door(_))
                },
                |_| 1,
                |_| 0
            ) {
                let target = *path.last().unwrap();
                if targets.iter().any(|(p,_c)| path.contains(p)) {
                    ignore.insert(target);
                    continue;
                }
                targets.insert(target, cost);

                let target_pos = big_graph.node_weight(target).unwrap().0;
                let bnode = nodes.get(&target_pos).unwrap().0;
                if graph.edges_connecting(anode, bnode).count() == 0 {
                    graph.add_edge(anode, bnode, cost);
                }
            }
        }
    }

    (graph, nodes)
}

pub mod part1 {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};
    use petgraph::visit::EdgeRef;

use crate::{Cell, convert_map_to_graph, parse};

    pub fn solve(file: String) -> usize {
        let (map, walker) = parse(file);
        let mut heap = BinaryHeap::new();
        let mut best = HashMap::new();
        let keys_len = map.iter().filter(|cell| matches!(cell, Cell::Key(_))).count();
        let all_keys = 2_u64.pow(keys_len as u32) - 1;

        let (graph, nodes) = convert_map_to_graph(&map);

        heap.push((Reverse(0),walker,0));

        while let Some((Reverse(step), pos, keys)) = heap.pop() {
            // If better history for state (position, keys holder), ignore this run
            if let Some(score) = best.get(&(pos, keys))
            && *score < step
            {
                // println!("{state:?} ignore {} < {}", score, state.step);
                continue;
            } else {
                best.insert((pos, keys), step);
            }
            
            // Got all keys end journey
            if keys == all_keys {
                return step;
            }
            if let Some((current_node,_)) = nodes.get(&pos)
            {
                let adjacents = graph.edges(*current_node);
                for edge in adjacents {
                    let cost = *edge.weight();
                    let next = edge.target();

                    if let Some((npos,ncel)) = graph.node_weight(next) {
                        let nkeys = match ncel {
                            Cell::Key(c) => keys | 1 << *c,
                            Cell::Door(c) if keys & 1 << *c == 0 => continue,
                            _ => keys
                        };
                        heap.push((Reverse(step + cost), *npos, nkeys));
                    }
                }
            }
        }
        0
    }
}

pub mod part2 {
    use std::collections::{BTreeSet, HashMap};

use crate::{Cell, parse};

    #[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
    pub struct State {
        step: usize,
        pos: [(usize,usize); 4],
        keys: u64,
    }

    pub fn solve(file: String) -> usize {
        let (mut map, walker) = parse(file);

        [(-1,0),(1,0),(0,0),(0,1),(0,-1)].iter().for_each(|(dx,dy)| {
            let cell = map.get_mut(
                (walker.1 as isize + dy) as usize,
                (walker.0 as isize + dx) as usize
            ).unwrap();
            *cell = Cell::Wall;
        });
        let mut walkers = [(0,0); 4];
        
        [(-1,-1),(-1,1),(1,-1),(1,1)].iter().enumerate().for_each(|(id,(dx,dy))| {
            walkers[id] = (
                (walker.0 as isize + dx) as usize,
                (walker.1 as isize + dy) as usize
            );
        });

        let mut stack = BTreeSet::new();
        let mut best = HashMap::new();
        let keys_len = map.iter().filter(|cell| matches!(cell, Cell::Key(_))).count();
        let all_keys = 2_u64.pow(keys_len as u32) - 1;
        stack.insert(State {
            step : 0,
            pos : walkers,
            keys : 0
        });

        while let Some(state) = stack.pop_first() {
            // If better history for state (position, keys holder), ignore this run
            if let Some(score) = best.get(&(state.pos, state.keys))
            && *score < state.step
            {
                continue;
            } else {
                best.insert((state.pos, state.keys), state.step);
            }
            
            // Got all keys end journey
            if state.keys == all_keys {
                continue;
            }

            // Select for each robot
            (0..4).for_each(|id| {
                let pos = state.pos[id];
                // adjacent cell able to walk only 4 axis, N E W S,
                [(-1,0),(1,0),(0,1),(0,-1)].iter().for_each(|(dx,dy)| {
                    let next_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
    
                    if (0..map.cols() as isize).contains(&next_pos.0)
                    && (0..map.rows() as isize).contains(&next_pos.1) {
                        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
                        let cell = map.get(next_pos.1, next_pos.0).expect("cannot find cell");
                        let mut next_walkers = state.pos;
                        next_walkers[id] = next_pos;
                        match cell {
                            Cell::Space | Cell::Start => {
                                stack.insert(State { step: state.step+1, pos: next_walkers, keys: state.keys });
                            },
                            Cell::Key(c) => {
                                stack.insert(State { step: state.step+1, pos: next_walkers, keys: state.keys | 1 << c});
                            },
                            Cell::Door(c) => {
                                // able to walk through the door, If got the pair key.
                                if state.keys & 1 << c != 0{
                                    stack.insert(State { step: state.step+1, pos: next_walkers, keys: state.keys });
                                }
                            },
                            Cell::Wall => {}
                        }
                    }
                });
            });
        }

        best.into_iter()
        .map(|((_pos,keys),step)| (step, keys))
        .filter(|(_step, keys)| *keys == all_keys)
        .min_by_key(|(step,_)| *step).unwrap().0
    }
}
