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
                let new_cell = new_map.get_mut(row, col).unwrap();
                if cell == &Cell::Open && nearby.iter().filter(|n| n == &&Cell::Tree).count() >= 3 {
                    *new_cell = Cell::Tree;
                } else if cell == &Cell::Tree
                    && nearby.iter().filter(|n| n == &&Cell::Lumberyard).count() >= 3
                {
                    *new_cell = Cell::Lumberyard;
                } else if cell == &Cell::Lumberyard {
                    if nearby.iter().filter(|n| n == &&Cell::Tree).count() >= 1
                        && nearby.iter().filter(|n| n == &&Cell::Lumberyard).count() >= 1
                    {
                        *new_cell = Cell::Lumberyard;
                    } else {
                        *new_cell = Cell::Open;
                    }
                } else {
                    *new_cell = *cell;
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
    use crate::{_display, Cell, parse, process};

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
    use std::collections::{HashMap, HashSet};

    use crate::{_display, Cell, parse, process};

    pub fn solve(file: String) -> usize {
        let mut map = parse(file);
        let mut time_line = HashMap::new();
        time_line.insert(0, map.clone());
        for i in 1..=1000 {
            map = process(&map);
            if time_line.values().find(|g| **g == map).is_none() {
                time_line.insert(i, map.clone());
            } else {
                break;
            }
        }

        let start_loop = *time_line.iter().find(|(_id, g)| **g == map).unwrap().0;
        let repeat = time_line.len() - start_loop;
        let target = 1_000_000_000_usize;
        let id = start_loop + (target - start_loop) % repeat;

        let map = time_line.get(&id).unwrap();
        let wood = map.iter().filter(|c| c == &&Cell::Tree).count();
        let yard = map.iter().filter(|c| c == &&Cell::Lumberyard).count();

        wood * yard
    }
}
