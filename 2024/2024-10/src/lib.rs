use grid::*;
use std::collections::HashSet;

pub fn parse(file: String) -> Grid<u32> {
    let cols = file
        .lines()
        .find(|line| !line.is_empty())
        .expect("cannot get first line")
        .len();
    let mut map = Grid::new(0, cols);

    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            map.push_row(
                line.chars()
                    .map(|c| c.to_string().parse::<u32>().expect("cannot parse digit"))
                    .collect(),
            );
        });

    map
}

pub fn find_highest(map: &Grid<u32>) -> Vec<(isize, isize)> {
    let cols = map.cols();
    map.iter()
        .enumerate()
        .filter_map(|(n, v)| {
            if *v == 9 {
                Some(((n / cols) as isize, (n % cols) as isize))
            } else {
                None
            }
        })
        .collect()
}

pub fn trail_down(map: &Grid<u32>, pos: (isize, isize)) -> Vec<(isize, isize)> {
    let current_pos = map.get(pos.0, pos.1);
    if current_pos.is_none() {
        return vec![];
    }

    let &value = current_pos.unwrap();
    [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .flat_map(|offset| {
            let next = (pos.0 + offset.0, pos.1 + offset.1);
            if let Some(&target) = map.get(next.0, next.1)
                && value - 1 == target
            {
                if target == 0 {
                    vec![next]
                } else {
                    trail_down(map, next)
                }
            } else {
                vec![]
            }
        })
        .collect()
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        let high_list = find_highest(&map);
        high_list
            .iter()
            .map(|&high| {
                let set: HashSet<(isize, isize)> = trail_down(&map, high).iter().copied().collect();
                set.len()
            })
            .sum::<usize>() as u64
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        let high_list = find_highest(&map);
        high_list
            .iter()
            .map(|&high| trail_down(&map, high).len())
            .sum::<usize>() as u64
    }
}
