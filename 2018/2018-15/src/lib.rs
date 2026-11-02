use std::collections::HashMap;

use grid::Grid;
use petgraph::graph::UnGraph;

macro_rules! debug_println {
    ($($arg:tt)*) => {
        {
            println!($($arg)*);
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    team: char,
    pos: (usize,usize),
    hp: u64
}

pub fn parse(file: String) -> (Grid<bool>, Vec<Unit>) {
    let mut map = Grid::new(0,0);
    let mut units = vec![];
    file.lines().enumerate().for_each(|(y,line)| {
        map.push_row(
            line.chars().enumerate().map(|(x,char)| {
                match char {
                    '#' => false,
                    '.' => true,
                    'E' => {
                        units.push(Unit{
                            team: 'E',
                            pos: (x,y),
                            hp: 200
                        });
                        true
                    },
                    'G' => {
                        units.push(Unit{
                            team: 'G',
                            pos: (x,y),
                            hp: 200
                        });
                        true
                    },
                    _ => unreachable!()
                }
            }).collect()
        );
    });

    (map, units)
}

pub fn get_graph_from_grid(grid: &Grid<bool>, units: &HashMap<usize, Unit>, start: usize) -> UnGraph<(usize,usize),usize> {
    let mut graph = UnGraph::new_undirected();
    let mut node_hash = HashMap::new();
    
    let cols = grid.cols();
    let rows = grid.rows();

    for row in 0..rows {
        for col in 0..cols {
            if let Some(cell) = grid.get(row, col)
            && *cell
            && ! units.iter().any(|(id,unit)| 
                *id != start && unit.pos.0 == col && unit.pos.1 == row
            )
            {
                node_hash.insert((col,row),graph.add_node((col,row)));
            }
        }
    }

    for ((x,y), node) in node_hash.iter() {
        let x = *x as isize;
        let y = *y as isize;
        for (dx,dy) in [(-1,0),(1,0),(0,-1),(0,1)] {
            if let Some(nearby) = node_hash.get(&((x + dx) as usize,(y + dy) as usize)){
                graph.add_edge(*node, *nearby, 
                    (1 *100000 + y * 200 + dy*100 + x * 2 + dx) as usize
                );
            }
        }
    }

    graph
}

pub fn _display(grid: &Grid<bool>, units: &HashMap<usize, Unit>) {
    let cols = grid.cols();
    let rows = grid.rows();

    for row in 0..rows {
        let mut life = "".to_string();
        for col in 0..cols {
            if let Some((_, unit)) = units.iter().find(|(_,unit)| {
                unit.pos == (col,row)
            }) {
                if unit.team == 'E' {
                    print!("E");
                    life += format!(" E({:3})",unit.hp).as_str();
                } else {
                    print!("G");
                    life += format!(" G({:3})",unit.hp).as_str();
                }
            } else {
                if let Some(cell) = grid.get(row, col) 
                && *cell
                {
                    print!(".");
                } else {
                    print!("#");
                }
            }
        }
        println!(" {life}");
    }
    println!();
}

pub mod part1 {
    use std::collections::{HashMap, HashSet};
    use petgraph::{algo::astar, visit::EdgeRef};
    use crate::{_display, get_graph_from_grid, parse};

    pub fn solve(file: String) -> u64 {
        let (grid, list) = parse(file);
        let mut list = list.into_iter().enumerate().collect::<HashMap<_,_>>();

        println!("Initially:");
        _display(&grid, &list);

        for turn in 1.. {
            // let mut deads_list = HashSet::new();
            let mut order = list.clone().into_iter().collect::<Vec<_>>();
            order.sort_by_key(|(_, unit)| {
                unit.pos.1 * 1000 + unit.pos.0
            });
            for (id,_unit) in order {
                let Some(unit) = list.get(&id).copied() else {
                    continue;
                };
                debug_println!(" {}#{id} {unit:?} -> ", unit.team);

                // find another target for unit with different team
                let mut enemys = Vec::new();
                list.iter().for_each(|(eid, enemy)| {
                    if unit.team != enemy.team {
                        enemys.push((*eid, *enemy));
                    }
                });

                // find pad to attack
                let mut pads = Vec::new();
                let temp_enemy = enemys.clone();
                
                for (_eid, e) in temp_enemy {
                    for (dx,dy) in [(0,-1),(0,1),(-1,0),(1,0)] {
                        let Some(x) = e.pos.0.checked_add_signed(dx) else { continue; };
                        let Some(y) = e.pos.1.checked_add_signed(dy) else { continue; };

                        if let Some(cell) = grid.get(y, x) 
                        && *cell
                        && ! enemys.iter().any(|(_id, ae)| ae.pos.0 == x && ae.pos.1 == y)
                        {
                            pads.push((x,y));
                        }
                    }
                }

                let graph = get_graph_from_grid(&grid, &list, id);
                let start_node = graph.node_indices().find(|node| {
                    let (x,y) = graph.node_weight(*node).unwrap();
                    *x == unit.pos.0 && *y == unit.pos.1
                }).unwrap();
                debug_println!("   Pads: {pads:?}");
                let pads_node = pads.iter().filter_map(|(px,py)| {
                    graph.node_indices().find(|node| {
                        let (x,y) = graph.node_weight(*node).unwrap();
                        x == px && y == py
                    })
                }).collect::<Vec<_>>();
                let path_result = 
                    astar(&graph, start_node, 
                        | target | {
                            pads_node.contains(&target)
                        },
                        |e| *graph.edge_weight(e.id()).unwrap(),
                        |_| 0
                    );

                if let Some((_n,path)) = path_result 
                && path.len() > 1
                {
                    let pos = path.iter().map(|node_id| {
                        *graph.node_weight(*node_id).unwrap()
                    }).collect::<Vec<_>>();
                    debug_println!("   Path: {pos:?}");
                    if let Some(unit) = list.get_mut(&id) {
                        unit.pos = pos[1];
                        debug_println!("   walk to {:?}", unit.pos);
                    }
                } else {
                    debug_println!("   no walk stay at {:?}", unit.pos);
                }

                // check enemy in attck range?
                let unit = list.get(&id).unwrap();
                let mut enemys = Vec::new();
                list.iter().for_each(|(eid, enemy)| {
                    if unit.team != enemy.team {
                        enemys.push((*eid, *enemy));
                    }
                });
                let mut in_range = enemys.iter().filter(|(_eid, enemy)| {
                    ([(0,-1),(0,1),(1,0),(-1,0)]).iter().any(|(dx,dy)| {
                        unit.pos.0 as isize + dx == enemy.pos.0 as isize && unit.pos.1 as isize + dy == enemy.pos.1 as isize
                    })
                }).collect::<Vec<_>>();
                if ! in_range.is_empty() {
                    // attack to less hp enemy first, then top-down left-right
                    in_range.sort_by_key(|(_id, e)| (e.hp * 100_000) + (e.pos.1 *100) as u64 + e.pos.0 as u64);
                    let target_id = in_range.first().unwrap().0;

                    let dead = if let Some(enemy) = list.get_mut(&target_id) {
                        debug_println!("   ⚔️ attack  {enemy:?}");
                        enemy.hp = enemy.hp.saturating_sub(3);
                        enemy.hp == 0
                    } else {
                        false
                    };
                    if dead {
                        debug_println!("   #{target_id} dead ☠️");
                        list.remove(&target_id);
                    }
                }
            }

            println!("-- after {turn} round");
            _display(&grid, &list);
            if list.iter().map(|(_, unit)| unit.team)
                .collect::<HashSet<char>>().len() == 1 {
                return turn as u64 * list.values().map(|unit| unit.hp).sum::<u64>();
            } 
        };
        0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
