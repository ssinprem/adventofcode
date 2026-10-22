use grid::Grid;

pub fn parse(file: String) -> Vec<(usize, usize)> {
    file.lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.trim().parse::<usize>().unwrap();
            let y = y.trim().parse::<usize>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>()
}

pub fn _display(map: &Grid<Option<usize>>) {
    let cols = map.cols();
    let rows = map.rows();

    for y in 0..rows {
        for x in 0..cols {
            if let Some(cell) = map.get(y, x)
                && let Some(val) = cell
            {
                print!("{val}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub mod part1 {
    use crate::parse;
    use grid::Grid;
    use std::collections::HashSet;

    fn find_nearby(origin: &[(usize, usize)], point: (usize, usize)) -> Option<usize> {
        let mut candidate = origin
            .iter()
            .enumerate()
            .map(|(id, (ox, oy))| {
                let dx = point.0.abs_diff(*ox);
                let dy = point.1.abs_diff(*oy);
                (id, (dx + dy))
            })
            .collect::<Vec<_>>();

        candidate.sort_by_key(|(_id, dist)| *dist);

        if candidate[0].1 != candidate[1].1 {
            Some(candidate[0].0)
        } else {
            None
        }
    }

    fn get_ignore_list(map: &Grid<Option<usize>>) -> HashSet<usize> {
        let mut ignore_origin: HashSet<usize> = HashSet::new();
        let cols = map.cols();
        let rows  = map.rows();
        for y in 0..rows {
            if let Some(left) = map.get(y, 0)
                && let Some(id) = left
            {
                ignore_origin.insert(*id);
            }
            if let Some(right) = map.get(y, cols - 1)
                && let Some(id) = right
            {
                ignore_origin.insert(*id);
            }
        }
        for x in 0..cols {
            if let Some(top) = map.get(0, x)
                && let Some(id) = top
            {
                ignore_origin.insert(*id);
            }
            if let Some(bottom) = map.get(rows - 1, x)
                && let Some(id) = bottom
            {
                ignore_origin.insert(*id);
            }
        }
        ignore_origin
    }

    pub fn solve(file: String) -> usize {
        let origin = parse(file);
        let max_x = origin.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = origin.iter().map(|(_, y)| *y).max().unwrap();

        let mut map: Grid<Option<usize>> = Grid::new(max_y + 2, max_x + 2);
        for y in 0..=max_y + 1 {
            for x in 0..=max_x + 1 {
                let cell = map.get_mut(y, x).unwrap();
                *cell = find_nearby(&origin, (x, y));
            }
        }

        let ignore_origin = get_ignore_list(&map);

        (0..(origin.len()))
            .into_iter()
            .map(|id| {
                if ignore_origin.contains(&id) {
                    0_usize
                } else {
                    map.iter().filter(|value| Some(id) == **value).count()
                }
            })
            .max()
            .unwrap()
    }
}

pub mod part2 {
    use crate::parse;
    use grid::Grid;

    fn get_all_dist(origin: &[(usize, usize)], point: (usize, usize)) -> usize {
        origin
            .iter()
            .map(|(ox, oy)| {
                let dx = point.0.abs_diff(*ox);
                let dy = point.1.abs_diff(*oy);
                dx + dy
            })
            .sum()
    }

    pub fn solve(file: String, max_dist: usize) -> usize {
        let origin = parse(file);
        let max_x = origin.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = origin.iter().map(|(_, y)| *y).max().unwrap();

        let mut map: Grid<usize> = Grid::new(max_y + 2, max_x + 2);
        for y in 0..=max_y + 1 {
            for x in 0..=max_x + 1 {
                let cell = map.get_mut(y, x).unwrap();
                *cell = get_all_dist(&origin, (x, y));
            }
        }

        map.iter().filter(|val| **val < max_dist).count()
    }
}
