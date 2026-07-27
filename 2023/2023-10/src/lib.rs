use grid::*;

#[derive(Debug, PartialEq, Clone)]
pub enum D {
    N,
    E,
    W,
    S,
    None,
}

pub fn ops(d: D) -> D {
    match d {
        D::N => D::S,
        D::E => D::W,
        D::W => D::E,
        D::S => D::N,
        _ => D::None,
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum Cell {
    Start, // S
    Pipe(D, D),
    #[default]
    Ground, // .
}

pub fn parse(file: String) -> Grid<Cell> {
    let mut maps = Grid::new(0, 0);
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let row = line
                .chars()
                .map(|char| match char {
                    'S' => Cell::Start,
                    '|' => Cell::Pipe(D::N, D::S),
                    '-' => Cell::Pipe(D::W, D::E),
                    'J' => Cell::Pipe(D::N, D::W),
                    'L' => Cell::Pipe(D::N, D::E),
                    '7' => Cell::Pipe(D::S, D::W),
                    'F' => Cell::Pipe(D::S, D::E),
                    _ => Cell::Ground,
                })
                .collect();
            maps.push_row(row);
        });
    maps
}

pub fn _display(maps: &Grid<Cell>, curr: (isize, isize)) {
    println!();
    maps.iter_rows().enumerate().for_each(|(y, row)| {
        for (x, cell) in row.enumerate() {
            if curr == (y as isize, x as isize) {
                print!(
                    "{}",
                    match *cell {
                        Cell::Start => "$",
                        Cell::Pipe(D::N, D::S) => "║",
                        Cell::Pipe(D::W, D::E) => "═",
                        Cell::Pipe(D::N, D::W) => "╝",
                        Cell::Pipe(D::N, D::E) => "╚",
                        Cell::Pipe(D::S, D::W) => "╗",
                        Cell::Pipe(D::S, D::E) => "╔",
                        Cell::Ground => ".",
                        _ => ".",
                    }
                );
            } else {
                print!(
                    "{}",
                    match *cell {
                        Cell::Start => "S",
                        Cell::Pipe(D::N, D::S) => "│",
                        Cell::Pipe(D::W, D::E) => "─",
                        Cell::Pipe(D::N, D::W) => "┘",
                        Cell::Pipe(D::N, D::E) => "└",
                        Cell::Pipe(D::S, D::W) => "┐",
                        Cell::Pipe(D::S, D::E) => "┌",
                        Cell::Ground => ".",
                        _ => ".",
                    }
                );
            }
        }
        println!();
    });
}

pub fn go_next(maps: &Grid<Cell>, mut curr: (isize,isize), mut from: D ) -> ((isize,isize),D) {
    if from == D::None {
        from = [D::N, D::E, D::W, D::S]
            .iter()
            .find(|d| {
                let diff = match d {
                    D::N => (-1, 0),
                    D::E => (0, 1),
                    D::W => (0, -1),
                    D::S => (1, 0),
                    _ => (0, 0),
                };
                if let Some(cell) = maps.get(curr.0 + diff.0, curr.1 + diff.1)
                    && let Cell::Pipe(a, b) = cell
                    && (from == D::None || *a == from || *b == from)
                {
                    true
                } else {
                    false
                }
            })
            .unwrap()
            .clone();
    }
    match from {
        D::N => {
            curr = (curr.0 - 1, curr.1);
        }
        D::E => {
            curr = (curr.0, curr.1 + 1);
        }
        D::W => {
            curr = (curr.0, curr.1 - 1);
        }
        D::S => {
            curr = (curr.0 + 1, curr.1);
        }
        _ => unreachable!(),
    };
    let next_cell = maps.get(curr.0, curr.1).unwrap();
    if let Cell::Pipe(a, b) = next_cell {
        if from == ops(a.clone()) {
            from = b.clone();
        } else if from == ops(b.clone()) {
            from = a.clone();
        } else {
            unreachable!()
        }
    }
    (curr, from)
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let maps = parse(file);
        let width = maps.cols();
        let start = maps.iter().position(|cell| *cell == Cell::Start).unwrap();
        let start = ((start / width) as isize, (start % width) as isize);
        let mut curr = start;
        _display(&maps, start);
        let mut count = 0;
        let mut from = D::None;
        while curr != start || count == 0 {
            (curr, from) = go_next(&maps, curr, from);
            count += 1;
        }
        count / 2
    }
}

pub mod part2 {
    use std::path;

use super::*;

    pub fn solve(file: String) -> u64 {
        let maps = parse(file);
        let width = maps.cols();
        let height = maps.rows();
        let start = maps.iter().position(|cell| *cell == Cell::Start).unwrap();
        let start = ((start / width) as isize, (start % width) as isize);
        let mut curr = start;
        _display(&maps, start);
        let mut path = Vec::<(isize,isize)>::new();
        let mut from = D::None;
        while curr != start || from == D::None {
            (curr, from) = go_next(&maps, curr, from);
            path.push(curr);
        }
        let mut path_map = maps;
        for row in 0..height as isize {
            for col in 0..width as isize {
                if path.iter().all(|(r,c)| (*r,*c) != (row,col)) {
                    let remove = path_map.get_mut(row, col).unwrap();
                    *remove = Cell::Ground
                }
            }
        }
        _display(&path_map, start);
        0
    }
}
