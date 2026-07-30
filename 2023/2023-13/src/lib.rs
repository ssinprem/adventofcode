use grid::Grid;

pub fn parse(file: String) -> Vec<Grid<bool>> {
    file.split("\n\n")
        .map(|pattern| {
            let mut maps = Grid::new(0, 0);
            pattern
                .lines()
                .filter(|line| !line.is_empty())
                .for_each(|line| {
                    maps.push_row(line.chars().map(|char| char == '#').collect());
                });
            maps
        })
        .collect()
}

pub fn is_vert_mirror(map: &Grid<bool>) -> Option<usize> {
    let cols = map.cols();
    for i in 0..cols - 1 {
        let mut j = 0;
        loop {
            let cola = map.iter_col(i - j);
            let colb = map.iter_col(i + 1 + j);
            if cola.zip(colb).all(|(a, b)| a == b) {
                j += 1;
            } else {
                break;
            }
            if i < j || i + 1 + j > cols - 1 {
                return Some(i + 1);
            }
        }
    }
    None
}

pub fn is_horz_mirror(map: &Grid<bool>) -> Option<usize> {
    let rows = map.rows();
    for i in 0..rows - 1 {
        let mut j = 0;
        loop {
            let rowa = map.iter_row(i - j);
            let rowb = map.iter_row(i + 1 + j);
            if rowa.zip(rowb).all(|(a, b)| a == b) {
                j += 1;
            } else {
                break;
            }
            if i < j || i + 1 + j > rows - 1 {
                return Some(i + 1);
            }
        }
    }
    None
}

pub mod part1 {
    use crate::{is_horz_mirror, is_vert_mirror, parse};

    pub fn solve(file: String) -> u64 {
        let maps = parse(file);

        maps.iter()
            .map(|map| {
                if let Some(u) = is_vert_mirror(map) {
                    u
                } else if let Some(u) = is_horz_mirror(map) {
                    u * 100
                } else {
                    0
                }
            })
            .sum::<usize>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
