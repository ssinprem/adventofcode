use grid::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn parse(file: String) -> Grid<char> {
    let cols = file
        .lines()
        .find(|line| !line.is_empty())
        .expect("cannot find first line")
        .chars()
        .count();
    let mut map = Grid::new(0, cols);

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            map.push_row(line.chars().collect());
        });
    map
}

#[allow(clippy::type_complexity)]
pub fn dijkstra(
    map: &Grid<char>,
    start_pos: (usize, usize),
    limit: u64,
    ignore_wall: bool,
) -> (
    HashMap<(usize, usize), u64>,
    HashMap<(usize, usize), HashSet<(usize, usize)>>,
) {
    let mut dists = HashMap::new();
    let mut pre_nodes = HashMap::<_, HashSet<_>>::new();
    let mut pq: BinaryHeap<(Reverse<u64>, (usize, usize))> = BinaryHeap::new();

    // init start pos with 0 score
    dists.insert(start_pos, 0);
    pq.push((Reverse(0), start_pos));

    while let Some((Reverse(score), pos)) = pq.pop() {
        if score > *dists.get(&pos).unwrap_or(&limit) && !ignore_wall {
            continue;
        }
        if score >= limit && ignore_wall{
            continue;
        }

        for div in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let next_pos = (
                (pos.0 as isize + div.0) as usize,
                (pos.1 as isize + div.1) as usize,
            );
            if next_pos.0 > 0
                && next_pos.0 < map.cols()
                && next_pos.1 > 0
                && next_pos.1 < map.rows()
                && (ignore_wall || map.get(next_pos.1, next_pos.0) != Some(&'#'))
            {
                let new_score = score + 1;
                let current_best = *dists.get(&next_pos).unwrap_or(&u64::MAX);
                if new_score < current_best {
                    dists.insert(next_pos, new_score);
                    pq.push((Reverse(new_score), next_pos));
                    pre_nodes.insert(next_pos, HashSet::from([pos]));
                } else if new_score == current_best {
                    pre_nodes.entry(next_pos).or_default().insert(pos);
                }
            }
        }
    }

    (dists, pre_nodes)
}

pub fn _print_map(map: &Grid<char>, steps: &[(isize, isize)]) {
    let cols = map.cols();
    let rows = map.rows();
    println!();
    for y in 0..rows {
        for x in 0..cols {
            if steps.contains(&(x as isize, y as isize)) {
                print!("O");
            } else {
                print!("{}", map.get(y, x).unwrap());
            }
        }
        println!();
    }
    println!();
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String, atleast: u64) -> u64 {
        let map = parse(file);
        let cols = map.cols();
        let rows = map.rows();
        let start = map
            .iter()
            .enumerate()
            .find_map(|(idx, char)| {
                if char == &'S' {
                    Some((idx % cols, idx / cols))
                } else {
                    None
                }
            })
            .expect("cannot find start point");
        let end = map
            .iter()
            .enumerate()
            .find_map(|(idx, char)| {
                if char == &'E' {
                    Some((idx % cols, idx / cols))
                } else {
                    None
                }
            })
            .expect("cannot find end point");

        let (basic_dist, _) = dijkstra(&map, start, u64::MAX, false);
        let basic_score = *basic_dist.get(&end).unwrap();
        println!("Basic score {basic_score}");
        _print_map(&map, &[]);
        let mut cheater_list = HashMap::new();
        for y in 0..rows {
            for x in 0..cols {
                if Some(&'#') == map.get(y, x) {
                    let mut mutate_map = map.clone();
                    let cell = mutate_map.get_mut(y, x).expect("cannot get cell");
                    *cell = '.';

                    let (mut_dist, _) = dijkstra(&mutate_map, start, basic_score, false);
                    let mut_score = mut_dist.get(&end).unwrap();
                    let cheat = basic_score - mut_score;
                    if cheat >= atleast {
                        println!("{x},{y} => {mut_score}");
                        cheater_list
                            .entry(cheat)
                            .and_modify(|c| *c += 1)
                            .or_insert(1);
                    }
                }
            }
        }
        println!("{cheater_list:?}");
        cheater_list.values().sum::<i32>() as u64
    }
}

pub mod part2 {
    use super::*;
    use std::collections::HashMap;

    pub fn solve(file: String, limit: u64) -> u64 {
        let map = parse(file);
        let cols = map.cols();
        let start = map
            .iter()
            .enumerate()
            .find_map(|(idx, char)| {
                if char == &'S' {
                    Some((idx % cols, idx / cols))
                } else {
                    None
                }
            })
            .expect("cannot find start point");
        let end = map
            .iter()
            .enumerate()
            .find_map(|(idx, char)| {
                if char == &'E' {
                    Some((idx % cols, idx / cols))
                } else {
                    None
                }
            })
            .expect("cannot find end point");

        let (dists_s, _) = dijkstra(&map, start, u64::MAX, false);
        let (dists_e, _) = dijkstra(&map, end, u64::MAX, false);
        let basic_score = *dists_s.get(&end).unwrap();
        println!("Basic score = {}", basic_score);
        let non_wall_cells: Vec<_> = map
            .indexed_iter()
            .filter_map(|((y, x), &c)| if c != '#' { Some((x, y)) } else { None })
            .collect();

        let mut cheater_list = HashMap::new();

        for &p1 in &non_wall_cells {
            let (cheat_dists, _) = dijkstra(&map, p1, 20, true);
            for ( p2, cheat_dist) in cheat_dists {
                if map.get(p2.1, p2.0) == Some(&'#') {
                    continue;
                }
                if p1 >= p2 {
                    // process each pair once
                    continue;
                }

                let saving1 = if let (Some(dist_s_p1), Some(dist_e_p2)) =
                    (dists_s.get(&p1), dists_e.get(&p2))
                {
                    basic_score as i64 - (*dist_s_p1 as i64 + cheat_dist as i64 + *dist_e_p2 as i64)
                } else {
                    0
                };

                let saving2 = if let (Some(dist_s_p2), Some(dist_e_p1)) =
                    (dists_s.get(&p2), dists_e.get(&p1))
                {
                    basic_score as i64 - (*dist_s_p2 as i64 + cheat_dist as i64 + *dist_e_p1 as i64)
                } else {
                    0
                };

                let best_saving = saving1.max(saving2) as u64;
                // println!("{best_saving}");
                if best_saving >= limit {
                    println!("{p1:?} {p2:?}  {best_saving}");
                    cheater_list
                        .entry(best_saving)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }

        println!("{:?}", cheater_list);
        cheater_list
            .iter()
            .map(|(_v, c)| *c)
            .sum::<i64>() as u64
    }
}
