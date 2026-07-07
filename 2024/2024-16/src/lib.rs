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

#[allow(clippy::type_complexity)]
pub fn cal_score(map: Grid<Cell>) -> HashMap<(isize, isize), (u64, HashSet<(isize, isize)>)> {
    let cols = map.cols() as isize;
    let start_idx = map
        .iter()
        .clone()
        .enumerate()
        .find(|(_id, cell)| cell == &&Cell::Start)
        .expect("cannot find start")
        .0 as isize;
    let start = (start_idx % cols, start_idx / cols);

    // node position -> score, set of previous node
    let mut scores: HashMap<(isize, isize), (u64, HashSet<(isize, isize)>)> = (0..map.rows())
        .flat_map(|x| {
            (0..map.cols())
                .filter_map(|y| {
                    if let Some(cell) = map.get(y, x)
                        && cell != &Cell::Wall
                    {
                        Some(((x as isize, y as isize), (u64::MAX, HashSet::from([]))))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<HashMap<_, _>>();
    let mut stack = BinaryHeap::new();
    // init start point with score 0
    scores.insert(start, (0, HashSet::from([])));
    stack.push((Reverse(0), start, (0, 0)));

    while let Some((Reverse(score), pos, dir)) = stack.pop() {
        if score
            > scores
                .get(&pos)
                .expect("cannot get score at current vertex")
                .0
        {
            continue;
        }
        for next_dir in [(0, -1), (1, 0), (-1, 0), (0, 1)] {
            // ignore backward directions
            if next_dir.0 == -dir.0 && next_dir.1 == -dir.1 {
                continue;
            }

            let next_pos = (pos.0 + next_dir.0, pos.1 + next_dir.1);
            // ignore wall
            if Some(&Cell::Wall) == map.get(next_pos.1, next_pos.0) {
                continue;
            }

            let turn = if dir != next_dir { 1000 } else { 0 };
            let new_score = score + turn + 1;

            stack.push((Reverse(new_score), next_pos, next_dir));
            if new_score
                <= scores
                    .get(&next_pos)
                    .expect("cannot get score at next_pos")
                    .0
            {
                scores.entry(next_pos).and_modify(|(score, prev)| {
                    if new_score == *score {
                        prev.insert(pos);
                    } else if new_score < *score {
                        *score = new_score;
                        *prev = HashSet::from([pos]);
                    }
                });
            }
        }
    }
    scores
}
pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        // _print_map(&map, &[]);
        let cols = map.cols() as isize;
        let end_idx = map
            .iter()
            .clone()
            .enumerate()
            .find(|(_id, cell)| cell == &&Cell::End)
            .expect("cannot find end")
            .0 as isize;
        let end = (end_idx % cols, end_idx / cols);

        let scores = cal_score(map);
        scores.get(&end).expect("cannot get end point").0
    }
}

pub mod part2 {
    use super::*;
    use std::collections::HashSet;

    fn score(path: &[(isize, isize)]) -> u64 {
        let turn = path
            .windows(3)
            .filter(|row| row[0].0 != row[2].0 && row[0].1 != row[2].1)
            .count()
            + 1;
        (turn * 1000 + path.len() - 1) as u64
    }
    pub fn find_path(
        map: &Grid<Cell>,
        path: Vec<(isize, isize)>,
        target: (isize, isize),
        best_score: u64,
    ) -> Option<Vec<Vec<(isize, isize)>>> {
        let score = score(&path);
        let curr = path.last().expect("cannot get current cell");
        let first = path.first().expect("cannot get first");
        let dist = (first.0 - target.0).abs() + (first.1 - target.1).abs();
        let progress = (curr.0 - target.0).abs() + (curr.1 - target.1).abs();
        let percent = (dist - progress)*1000/dist;
        // println!("{score}/{best_score} -- {} > {percent}",score*1000/best_score);
        if  score*1000/best_score > percent as u64 * 4 && percent !=0 {
            return None;
        }
        
        if score > best_score {
            return None;
        }
        if curr == &target && score == best_score {
            _print_map(map, &path);
            println!("beat score : {score}");
            return Some(vec![path]);
        }
        
        let paths: Vec<Vec<(isize, isize)>> = ([(-1, 0), (0, -1), (1, 0), (0, 1)])
            .iter()
            .filter_map(|direction| {
                let next = (curr.0 + direction.0, curr.1 + direction.1);
                // ignore the old line
                if path.contains(&next) {
                    return None;
                }
                // ignore wall
                if let Some(item) = map.get(next.1, next.0)
                    && item == &Cell::Wall
                {
                    return None;
                }
                let mut next_path = path.clone();
                next_path.push(next);
                // ignore if score worse then the best score.
                find_path(map, next_path, target, best_score)
            })
            .flatten()
            .collect();

        if paths.is_empty() { None } else { Some(paths) }
    }

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        // _print_map(&map, &[]);
        let cols = map.cols() as isize;
        let start_idx = map
            .iter()
            .clone()
            .enumerate()
            .find(|(_id, cell)| cell == &&Cell::Start)
            .expect("cannot find start")
            .0 as isize;
        let start = (start_idx % cols, start_idx / cols);
        let end_idx = map
            .iter()
            .clone()
            .enumerate()
            .find(|(_id, cell)| cell == &&Cell::End)
            .expect("cannot find end")
            .0 as isize;
        let end = (end_idx % cols, end_idx / cols);

        let scores = cal_score(map.clone());
        let best = scores.get(&end).expect("cannot get end point");

        let mut uniq_nodes: HashSet<(isize, isize)> = HashSet::<(isize, isize)>::new();
        // let mut stack = vec![best];
        uniq_nodes.insert(end);
        // while let Some(node) = stack.pop() {
        //     for &pre_node in node.1.iter() {
        //         uniq_nodes.insert(pre_node);
        //         stack.push(
        //             scores.get(&pre_node).expect("get pre node")
        //         );
        //     }
        // }
        let paths = find_path(&map, vec![start], end, best.0).expect("path generate failed");
        for path in paths.clone() {
            for cell in path {
                uniq_nodes.insert(cell);
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
