use grid::Grid;

const MOD: usize = 20183;
const X_MUL: usize = 16807;
const Y_MUL: usize = 48271;
pub fn parse(file: String) -> (usize, (usize, usize)) {
    use regex::Regex;
    let regex_dept = Regex::new(r"depth: (\d+)").unwrap();
    let dept = regex_dept
        .captures(file.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let regex_pos = Regex::new(r"target: (\d+),(\d+)").unwrap();
    let match_pos = regex_pos.captures(file.as_str()).unwrap();
    let x = match_pos.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let y = match_pos.get(2).unwrap().as_str().parse::<usize>().unwrap();
    (dept, (x, y))
}

pub fn create_grid(dept: usize, target: (usize, usize)) -> Grid<usize> {
    let mut map = Grid::new(target.1 + 1, target.0 + 1);

    let rows = map.rows();
    let cols = map.cols();

    for row in 0..rows {
        for col in 0..cols {
            if (col, row) == (0, 0) {
                let cell = map.get_mut(row, col).unwrap();
                *cell = dept % MOD;
            } else if row == 0 {
                let cell = map.get_mut(0, col).unwrap();
                *cell = ((col * X_MUL) + dept) % MOD;
            } else if col == 0 {
                let cell = map.get_mut(row, 0).unwrap();
                *cell = ((row * Y_MUL) + dept) % MOD;
            } else {
                let value = map.get(row, col - 1).unwrap() * map.get(row - 1, col).unwrap();
                let value = (value + dept) % MOD;
                let cell = map.get_mut(row, col).unwrap();
                *cell = value;
            }
        }
    }

    map.map(|x| x % 3).clone()
}

pub fn _display(grid: &Grid<usize>, target: (usize, usize)) {
    let rows = grid.rows();
    let cols = grid.cols();

    for row in 0..rows {
        for col in 0..cols {
            if (col, row) == (0, 0) {
                print!("M");
            } else if (col, row) == target {
                print!("T");
            } else {
                let level = *grid.get(row, col).unwrap();
                match level {
                    0 => print!("."),
                    1 => print!("="),
                    2 => print!("|"),
                    _ => print!("?"),
                }
            }
        }
        println!();
    }
}

pub mod part1 {

    use crate::{_display, create_grid, parse};

    pub fn solve(file: String) -> usize {
        let (dept, target) = parse(file);
        let grid = create_grid(dept, target);

        _display(&grid, target);

        let mut vals = 0;
        for row in 0..=target.1 {
            for col in 0..=target.0 {
                let val = if target != (col, row) {
                    *grid.get(row, col).unwrap()
                } else {
                    0
                };
                vals += val;
            }
        }

        vals
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
