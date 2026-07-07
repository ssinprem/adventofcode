use grid::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Cell {
    #[default]
    Unknown,
    Wall,
    Path,
    Start,
    End,
}

fn _pause() {
    use std::io;
    use std::io::prelude::*;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn parse(file: String) -> Grid<Cell> {
    let cols = file.lines().find(|line| !line.is_empty()).iter().len();
    let mut map = Grid::new(0, cols);

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            map.push_row(
                line.chars()
                    .map(|c| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Path,
                        'S' => Cell::Start,
                        'E' => Cell::End,
                        _ => Cell::Unknown,
                    })
                    .collect(),
            );
        });
    map
}

pub fn _print_map(map: &Grid<Cell>, path: &[(isize, isize)]) {
    let cols = map.cols();
    let rows = map.rows();
    for y in 0..rows {
        for x in 0..cols {
            print!(
                "{}",
                match (
                    map.get(y, x),
                    path.iter()
                        .any(|(px, py)| *py == y as isize && *px == x as isize)
                ) {
                    (Some(Cell::Start), _) => "S",
                    (Some(Cell::End), _) => "E",
                    (Some(Cell::Path), false) => ".",
                    (Some(Cell::Path), true) => "O",
                    (Some(Cell::Wall), _) => "#",
                    _ => "?",
                }
            );
        }
        println!();
    }
}

// Helper functions for directions
fn turn_left(dir: (isize, isize)) -> (isize, isize) {
    (dir.1, -dir.0)
}

fn turn_right(dir: (isize, isize)) -> (isize, isize) {
    (-dir.1, dir.0)
}

// State-based Dijkstra
// Returns:
// 1. Map of minimum distances to each (position, direction) state
// 2. Map of previous nodes for each (position, direction) state
fn cal_score(
    map: &Grid<Cell>,
    start_pos: (isize, isize),
) -> (
    HashMap<((isize, isize), (isize, isize)), u64>,
    HashMap<((isize, isize), (isize, isize)), HashSet<((isize, isize), (isize, isize))>>,
) {
    let mut dists = HashMap::new();
    let mut pre_nodes: HashMap<_, HashSet<_>> = HashMap::new();
    let mut pq: BinaryHeap<(Reverse<u64>, (isize, isize), (isize, isize))> = BinaryHeap::new();

    // Start facing East (1, 0)
    let start_state = (start_pos, (1, 0));
    dists.insert(start_state, 0);
    pq.push((Reverse(0), start_pos, (1, 0)));

    while let Some((Reverse(score), pos, dir)) = pq.pop() {
        if score > *dists.get(&(pos, dir)).unwrap_or(&u64::MAX) {
            continue;
        }

        // Action 1: Move forward
        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if map.get(next_pos.1, next_pos.0) != Some(&Cell::Wall)
        {
            let next_state = (next_pos, dir);
            let new_score = score + 1;
            let current_best = *dists.get(&next_state).unwrap_or(&u64::MAX);

            if new_score < current_best {
                dists.insert(next_state, new_score);
                pq.push((Reverse(new_score), next_pos, dir));
                pre_nodes.insert(next_state, HashSet::from([(pos, dir)]));
            } else if new_score == current_best {
                pre_nodes.entry(next_state).or_default().insert((pos, dir));
            }
        }

        // Action 2 & 3: Turn Left and Right (each turn costs 1000)
        for next_dir in [turn_left(dir), turn_right(dir)] {
            let next_state = (pos, next_dir);
            let new_score = score + 1000;
            let current_best = *dists.get(&next_state).unwrap_or(&u64::MAX);

            if new_score < current_best {
                dists.insert(next_state, new_score);
                pq.push((Reverse(new_score), pos, next_dir));
                pre_nodes.insert(next_state, HashSet::from([(pos, dir)]));
            } else if new_score == current_best {
                pre_nodes.entry(next_state).or_default().insert((pos, dir));
            }
        }
    }

    (dists, pre_nodes)
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        let cols = map.cols() as isize;
        let start_idx = map.iter().position(|c| c == &Cell::Start).unwrap() as isize;
        let end_idx = map.iter().position(|c| c == &Cell::End).unwrap() as isize;
        let start_pos = (start_idx % cols, start_idx / cols);
        let end_pos = (end_idx % cols, end_idx / cols);

        let (dists, _) = cal_score(&map, start_pos);
        
        // The answer is the minimum score to reach the end position in any of the 4 directions
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .filter_map(|&dir| dists.get(&(end_pos, dir)).copied())
            .min()
            .unwrap_or(u64::MAX)
    }
}

pub mod part2 {
    use super::*;
    use std::collections::{HashSet, VecDeque};

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        let cols = map.cols() as isize;
        let start_idx = map.iter().position(|c| c == &Cell::Start).unwrap() as isize;
        let end_idx = map.iter().position(|c| c == &Cell::End).unwrap() as isize;
        let start_pos = (start_idx % cols, start_idx / cols);
        let end_pos = (end_idx % cols, end_idx / cols);

        let (dists, preds) = cal_score(&map, start_pos);

        // Find the best score to reach the end
        let best_score = [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .filter_map(|&dir| dists.get(&(end_pos, dir)).copied())
            .min()
            .unwrap_or(u64::MAX);

        if best_score == u64::MAX {
            return 0;
        }

        // Start backward search from all end states that have the best score
        let mut queue = VecDeque::new();
        let mut visited_states = HashSet::new();
        let mut uniq_nodes = HashSet::new();

        for &dir in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let state = (end_pos, dir);
            if dists.get(&state) == Some(&best_score) {
                queue.push_back(state);
                visited_states.insert(state);
            }
        }

        while let Some(state) = queue.pop_front() {
            uniq_nodes.insert(state.0); // state.0 is the position

            if let Some(previous_nodes) = preds.get(&state) {
                for &pred in previous_nodes {
                    if visited_states.insert(pred) {
                        queue.push_back(pred);
                    }
                }
            }
        }

        _print_map(
            &map,
            uniq_nodes
                .iter()
                .copied()
                .collect::<Vec<(isize, isize)>>()
                .as_slice(),
        );

        uniq_nodes.len() as u64
    }
}
