use grid::Grid;
use petgraph::graph::UnGraph;
use std::collections::{HashMap, HashSet, VecDeque};

macro_rules! debug_println {
    ($($arg:tt)*) => {{
        // println!($($arg)*);
    }};
}

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    team: char,
    pos: (usize, usize),
    hp: u64,
    atk: u64,
}

pub fn parse(file: String) -> (Grid<bool>, Vec<Unit>) {
    let mut map = Grid::new(0, 0);
    let mut units = vec![];
    file.lines().enumerate().for_each(|(y, line)| {
        map.push_row(
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '#' => false,
                    '.' => true,
                    'E' => {
                        units.push(Unit {
                            team: 'E',
                            pos: (x, y),
                            hp: 200,
                            atk: 3,
                        });
                        true
                    }
                    'G' => {
                        units.push(Unit {
                            team: 'G',
                            pos: (x, y),
                            hp: 200,
                            atk: 3,
                        });
                        true
                    }
                    _ => unreachable!(),
                })
                .collect(),
        );
    });

    (map, units)
}

pub fn get_graph_from_grid(
    grid: &Grid<bool>,
    units: &HashMap<usize, Unit>,
    start: usize,
) -> UnGraph<(usize, usize), usize> {
    let mut graph = UnGraph::new_undirected();
    let mut node_hash = HashMap::new();

    let cols = grid.cols();
    let rows = grid.rows();

    for row in 0..rows {
        for col in 0..cols {
            if let Some(cell) = grid.get(row, col)
                && *cell
                && !units
                    .iter()
                    .any(|(id, unit)| *id != start && unit.pos.0 == col && unit.pos.1 == row)
            {
                node_hash.insert((col, row), graph.add_node((col, row)));
            }
        }
    }

    for ((x, y), node) in node_hash.iter() {
        let x = *x as isize;
        let y = *y as isize;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some(nearby) = node_hash.get(&((x + dx) as usize, (y + dy) as usize)) {
                graph.add_edge(*node, *nearby, 1);
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
            if let Some((_, unit)) = units.iter().find(|(_, unit)| unit.pos == (col, row)) {
                if unit.team == 'E' {
                    print!("E");
                    life += format!(" E({:3})", unit.hp).as_str();
                } else {
                    print!("G");
                    life += format!(" G({:3})", unit.hp).as_str();
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

pub fn bfs(
    start: (usize, usize),
    end: (usize, usize),
    grid: &Grid<bool>,
    units: &HashMap<usize, Unit>,
) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, vec![]));
    visited.insert(start);

    while let Some((pos, path)) = queue.pop_front() {
        if pos == end {
            return path;
        }
        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let Some(nx) = pos.0.checked_add_signed(dx) else {
                continue;
            };
            let Some(ny) = pos.1.checked_add_signed(dy) else {
                continue;
            };

            let next_pos = (nx, ny);

            if visited.contains(&next_pos) {
                continue;
            }

            if let Some(cell) = grid.get(ny, nx)
                && *cell
                && !units.values().any(|unit| unit.pos == next_pos)
            {
                visited.insert(next_pos);
                queue.push_back((next_pos, [path.clone(), vec![next_pos]].concat()));
            }
        }
    }
    vec![]
}

pub fn lets_fight(units: &mut HashMap<usize, Unit>, grid: &Grid<bool>, round: u64) -> Option<u64> {
    use petgraph::algo::astar;

    // re-order by current position  top->down  left->right
    let mut order = units.clone().into_iter().collect::<Vec<_>>();
    order.sort_by_key(|(_, unit)| unit.pos.1 * 1000 + unit.pos.0);
    for (id, _unit) in order {
        let Some(unit) = units.get(&id).copied() else {
            continue;
        };
        debug_println!(" {}#{id} {unit:?} -> ", unit.team);

        // find another target for unit with different team
        let mut enemys = Vec::new();
        units.iter().for_each(|(eid, enemy)| {
            if unit.team != enemy.team {
                enemys.push((*eid, *enemy));
            }
        });
        // when no enemy left !! End Game !!
        if enemys.is_empty() {
            return Some(round * units.values().map(|unit| unit.hp).sum::<u64>());
        }

        // find pad to attack
        let mut pads = Vec::new();
        let temp_enemy = enemys.clone();
        'find_pad: for (_eid, e) in temp_enemy {
            for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let Some(x) = e.pos.0.checked_add_signed(dx) else {
                    continue;
                };
                let Some(y) = e.pos.1.checked_add_signed(dy) else {
                    continue;
                };

                if unit.pos == (x, y) {
                    // already nearby to attack
                    pads = vec![];
                    break 'find_pad;
                }

                if let Some(cell) = grid.get(y, x)
                    && *cell
                    && (!units.iter().any(|(_id, ae)| ae.pos.0 == x && ae.pos.1 == y)
                        || unit.pos == (x, y))
                {
                    pads.push((x, y));
                }
            }
        }
        if !pads.is_empty() {
            pads.sort_by_key(|(x, y)| (*y, *x));
            let graph = get_graph_from_grid(grid, units, id);
            let start_node = graph
                .node_indices()
                .find(|node| {
                    let (x, y) = graph.node_weight(*node).unwrap();
                    *x == unit.pos.0 && *y == unit.pos.1
                })
                .unwrap();
            debug_println!("   Pads: {pads:?}");
            let pads_node = pads
                .iter()
                .filter_map(|(px, py)| {
                    graph.node_indices().find(|node| {
                        let (x, y) = graph.node_weight(*node).unwrap();
                        x == px && y == py
                    })
                })
                .collect::<Vec<_>>();
            let path_result = astar(
                &graph,
                start_node,
                |target| pads_node.contains(&target),
                |_e| 1,
                |_| 0,
            );

            if let Some((best, _path)) = path_result
                && best > 0
            {
                let mut candidate = pads_node
                    .iter()
                    .filter(|target_node| {
                        if let Some((cost, _path)) = astar(
                            &graph,
                            start_node,
                            |target| target == **target_node,
                            |_e| 1,
                            |_| 0,
                        ) {
                            cost == best
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<_>>();

                candidate.sort_by_key(|last_node| {
                    let last_pos = graph.node_weight(**last_node).unwrap();
                    (last_pos.1, last_pos.0)
                });
                let target_node = candidate.first().unwrap();
                let target_pos = graph.node_weight(**target_node).unwrap();
                let path = bfs(unit.pos, *target_pos, grid, units);

                debug_println!("   Path: {path:?}");
                if let Some(unit) = units.get_mut(&id) {
                    unit.pos = path[0];
                    debug_println!("   walk to {:?}", unit.pos);
                }
            }
        }

        // check enemy in attck range?
        let unit = units.get(&id).unwrap();
        let mut enemys = Vec::new();
        units.iter().for_each(|(eid, enemy)| {
            if unit.team != enemy.team {
                enemys.push((*eid, *enemy));
            }
        });
        let mut in_range = enemys
            .iter()
            .filter(|(_eid, enemy)| {
                ([(0, -1), (0, 1), (1, 0), (-1, 0)]).iter().any(|(dx, dy)| {
                    unit.pos.0 as isize + dx == enemy.pos.0 as isize
                        && unit.pos.1 as isize + dy == enemy.pos.1 as isize
                })
            })
            .collect::<Vec<_>>();
        if !in_range.is_empty() {
            // attack to less hp enemy first, then top-down left-right
            in_range.sort_by_key(|(_id, e)| (e.hp, e.pos.1, e.pos.0));
            let target_id = in_range.first().unwrap().0;
            let atk = unit.atk;

            let dead = if let Some(enemy) = units.get_mut(&target_id) {
                debug_println!("   ⚔️ attack  {enemy:?}");
                enemy.hp = enemy.hp.saturating_sub(atk);
                enemy.hp == 0
            } else {
                false
            };
            if dead {
                debug_println!("   #{target_id} dead ☠️");
                units.remove(&target_id);
            }
        }
    }
    None
}

pub mod part1 {
    use crate::{_display, lets_fight, parse};
    use std::collections::HashMap;

    pub fn solve(file: String) -> u64 {
        let (grid, units) = parse(file);
        let mut units = units.into_iter().enumerate().collect::<HashMap<_, _>>();

        println!("Initially:");
        _display(&grid, &units);
        let mut full_rounds: u64 = 0;
        loop {
            if let Some(outcome) = lets_fight(&mut units, &grid, full_rounds) {
                println!("-- (last turn) after {full_rounds} round");
                _display(&grid, &units);
                return outcome;
            }
            full_rounds += 1;
        }
    }
}
pub mod part2 {
    use crate::{_display, lets_fight, parse};
    use std::collections::HashMap;

    pub fn solve(file: String) -> u64 {
        let (grid, units) = parse(file);
        let save_units = units.into_iter().enumerate().collect::<HashMap<_, _>>();

        let elf_num = save_units
            .iter()
            .filter(|(_, unit)| unit.team == 'E')
            .count();
        for elf_atk in 4.. {
            let mut full_rounds: u64 = 0;
            let mut units = save_units
                .clone()
                .iter()
                .map(|(id, unit)| {
                    let mut unit = *unit;
                    if unit.team == 'E' {
                        unit.atk = elf_atk;
                    } else {
                        unit.atk = 3;
                    }
                    (*id, unit)
                })
                .collect::<HashMap<_, _>>();
            loop {
                if let Some(outcome) = lets_fight(&mut units, &grid, full_rounds) {
                    println!(
                        "-- With elf atk {elf_atk} - (last turn) after {full_rounds} round"
                    );
                    _display(&grid, &units);
                    // Elf win without single death
                    if units.iter().filter(|(_, unit)| unit.team == 'E').count() == elf_num {
                        _display(&grid, &save_units);

                        _display(&grid, &units);
                        return outcome;
                    }
                    break;
                }
                full_rounds += 1;
            }
        }
        0
    }
}
