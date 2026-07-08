use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use grid::*;

pub fn parse(file: String, size: (usize, usize), limit: usize) -> Grid<char> {
    let mut map = Grid::<char>::new(size.1, size.0);

    map.fill('.');

    file.lines()
        .filter(|line| !line.is_empty())
        .take(limit)
        .for_each(|line| {
            if let Some((str_x, str_y)) = line.split_once(",") {
                let x: usize = str_x.parse().unwrap();
                let y: usize = str_y.parse().unwrap();
                let cell = map.get_mut(y, x).expect("cannot edit map grid value");
                *cell = '#';
            }
        });

    map
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

#[allow(clippy::type_complexity)]
pub fn dijkstra(
    map: &Grid<char>,
    start_pos: (isize, isize),
) -> (
    HashMap<(isize, isize), u64>,
    HashMap<(isize, isize), HashSet<(isize, isize)>>,
) {
    let mut dists = HashMap::new();
    let mut pre_nodes = HashMap::<_, HashSet<_>>::new();
    let mut pq: BinaryHeap<(Reverse<u64>, (isize, isize))> = BinaryHeap::new();

    // init start pos with 0 score
    dists.insert(start_pos, 0);
    pq.push((Reverse(0), start_pos));

    while let Some((Reverse(score), pos)) = pq.pop() {
        if score > *dists.get(&pos).unwrap_or(&u64::MAX) {
            continue;
        }

        for div in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let next_pos = (pos.0 + div.0, pos.1 + div.1);
            if map.get(next_pos.1, next_pos.0) == Some(&'.') {
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

pub mod part1 {
    use super::*;

    pub fn solve(file: String, size: (usize, usize), limit: usize) -> u64 {
        let map = parse(file, size, limit);
        _print_map(&map, &[]);
        let size = (size.0 as isize, size.1 as isize);
        let (dist, pre_nodes) = dijkstra(&map, (0, 0));

        let mut stack = Vec::new();
        let mut steps = Vec::new();
        stack.push((size.0 - 1, size.1 - 1));
        steps.push((size.0 - 1, size.1 - 1));
        while let Some(node) = stack.pop() {
            if let Some(pre_node) = pre_nodes.get(&node)
                && let Some(n) = pre_node.iter().next()
            {
                stack.push(*n);
                steps.push(*n);
            }
        }
        _print_map(&map, &steps);

        *dist.get(&(size.0 - 1, size.1 - 1)).unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
