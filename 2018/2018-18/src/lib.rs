use grid::Grid;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    #[default]
    Open,
    Tree,
    Lumberyard,
}

pub fn parse(file: String) -> Grid<Cell> {
    let mut map = Grid::new(0, 0);
    file.lines().for_each(|line| {
        map.push_row(
            line.chars()
                .map(|char| match char {
                    '.' => Cell::Open,
                    '|' => Cell::Tree,
                    '#' => Cell::Lumberyard,
                    _ => unreachable!(),
                })
                .collect(),
        );
    });
    map
}

pub fn process(map: &Grid<Cell>) -> Grid<Cell> {
    let rows = map.rows();
    let cols = map.cols();
    let mut new_map = map.clone();
    for row in 0..rows {
        for col in 0..cols {
            if let Some(cell) = map.get(row, col) {
                let nearby = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .iter()
                .filter_map(|(dx, dy)| {
                    map.get((*dy + row as isize) as usize, (*dx + col as isize) as usize)
                })
                .copied()
                .collect::<Vec<Cell>>();

                let trees = nearby.iter().filter(|n| n == &&Cell::Tree).count();
                let yards = nearby.iter().filter(|n| n == &&Cell::Lumberyard).count();
                new_map[(row, col)] = match cell {
                    Cell::Open if trees >= 3 => Cell::Tree,
                    Cell::Tree if yards >= 3 => Cell::Lumberyard,
                    Cell::Lumberyard if trees == 0 || yards == 0 => Cell::Open,
                    c => *c,
                }
            }
        }
    }
    new_map
}

pub fn _display(map: &Grid<Cell>) {
    let rows = map.rows();
    let cols = map.cols();

    for row in 0..rows {
        for col in 0..cols {
            if let Some(cell) = map.get(row, col) {
                print!(
                    "{}",
                    match cell {
                        Cell::Open => ".",
                        Cell::Tree => "|",
                        Cell::Lumberyard => "#",
                    }
                );
            }
        }
        println!();
    }
    println!();
}

pub mod part1 {
    use crate::{Cell, parse, process};

    pub fn solve(file: String) -> usize {
        let mut map = parse(file);

        for _i in 1..=10 {
            map = process(&map);
        }
        let wood = map.iter().filter(|c| c == &&Cell::Tree).count();
        let yard = map.iter().filter(|c| c == &&Cell::Lumberyard).count();

        wood * yard
    }
}

pub mod part2 {
    use crate::{Cell, parse, process};

    pub fn solve(file: String) -> usize {
        let mut map = parse(file);
        let mut history = vec![map.clone()];
        let start_loop = loop {
            map = process(&map);
            if let Some(pos) = history.iter().position(|g| *g == map) {
                break pos;
            }
            history.push(map.clone());
        };
        let repeat = history.len() - start_loop;
        let map = &history[start_loop + (1_000_000_000 - start_loop) % repeat];
        let wood = map.iter().filter(|c| c == &&Cell::Tree).count();
        let yard = map.iter().filter(|c| c == &&Cell::Lumberyard).count();

        wood * yard
    }
}
