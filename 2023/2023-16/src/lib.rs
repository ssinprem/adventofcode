use grid::*;
use rayon::prelude::*;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty,
    SplitV,
    SplitH,
    MirrorB,
    MirrorF,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    Up = 1,
    Left = 2,
    Down = 4,
    Right = 8,
}

pub fn parse(file: String) -> Grid<(Cell, u8, u8)> {
    let mut map = Grid::new(0, 0);
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            map.push_row(
                line.chars()
                    .map(|char| match char {
                        '|' => (Cell::SplitV, 0, 0),
                        '-' => (Cell::SplitH, 0, 0),
                        '\\' => (Cell::MirrorB, 0, 0),
                        '/' => (Cell::MirrorF, 0, 0),
                        _ => (Cell::Empty, 0, 0),
                    })
                    .collect(),
            );
        });

    map
}

pub fn _display(map: &Grid<(Cell, u8, u8)>) -> String {
    let mut string = String::new();

    let rows = map.rows();
    let cols = map.cols();

    for row in 0..rows {
        for i in 0..3 {
            for col in 0..cols {
                for j in 0..3 {
                    if i == 0 && j == 1 {
                        if let Some((_, from, to)) = map.get(row, col) {
                            string += match (from & Dir::Up as u8 != 0, to & Dir::Up as u8 != 0) {
                                (true, true) => "⇅",
                                (true, false) => "↓",
                                (false, true) => "↑",
                                _ => " ",
                            };
                        } else {
                            string += " ";
                        }
                    } else if i == 2 && j == 1 {
                        if let Some((_, from, to)) = map.get(row, col) {
                            string += match (from & Dir::Down as u8 != 0, to & Dir::Down as u8 != 0)
                            {
                                (true, true) => "⇅",
                                (true, false) => "↑",
                                (false, true) => "↓",
                                _ => " ",
                            };
                        } else {
                            string += " ";
                        }
                    } else if i == 1 && j == 0 {
                        if let Some((_, from, to)) = map.get(row, col) {
                            string += match (from & Dir::Left as u8 != 0, to & Dir::Left as u8 != 0)
                            {
                                (true, true) => "⇄",
                                (true, false) => "→",
                                (false, true) => "←",
                                _ => " ",
                            };
                        } else {
                            string += " ";
                        }
                    } else if i == 1 && j == 2 {
                        if let Some((_, from, to)) = map.get(row, col) {
                            string +=
                                match (from & Dir::Right as u8 != 0, to & Dir::Right as u8 != 0) {
                                    (true, true) => "⇄",
                                    (true, false) => "←",
                                    (false, true) => "→",
                                    _ => " ",
                                };
                        } else {
                            string += " ";
                        }
                    } else if i == 1 && j == 1 {
                        match map.get(row, col) {
                            Some((Cell::SplitH, _, _)) => string += "-",
                            Some((Cell::SplitV, _, _)) => string += "|",
                            Some((Cell::MirrorB, _, _)) => string += "\\",
                            Some((Cell::MirrorF, _, _)) => string += "/",
                            _ => string += ".",
                        }
                    } else if let Some((_, from, _)) = map.get(row, col)
                        && *from != 0
                    {
                        string += match (i, j) {
                            (0, 0) => "┌",
                            (0, 2) => "┐",
                            (2, 0) => "└",
                            (2, 2) => "┘",
                            _ => " ",
                        };
                    } else {
                        string += " ";
                    }
                }
            }
            string += "\n";
        }
    }
    for _col in 0..cols {
        string += "===";
    }
    string
}

pub fn add(
    rays: &mut Vec<(i32, i32, Dir)>,
    history: &mut Vec<(i32, i32, Dir)>,
    item: (i32, i32, Dir),
) {
    if !rays.contains(&item) && !history.contains(&item) {
        rays.push(item);
        history.push(item);
    }
}

pub fn process(map: &mut Grid<(Cell, u8, u8)>, starts: Vec<(i32, i32, Dir)>) {
    let mut rays = Vec::new();
    let mut history = Vec::new();
    //    1
    // 2     8
    //    4
    for start in starts {
        add(&mut rays, &mut history, start);
    }
    while let Some((row, col, from)) = rays.pop() {
        if let Some((cell, cfrom, cto)) = map.get_mut(row, col) {
            // if cell.1 & from as u8 != 0 {
            //     continue;
            // }
            *cfrom |= from as u8;
            let mut to = *cto;
            if *cell == Cell::Empty {
                //  '.'
                match from {
                    Dir::Up => {
                        add(&mut rays, &mut history, (row + 1, col, Dir::Up));
                        to |= Dir::Down as u8;
                    }
                    Dir::Left => {
                        add(&mut rays, &mut history, (row, col + 1, Dir::Left));
                        to |= Dir::Right as u8;
                    }
                    Dir::Down => {
                        add(&mut rays, &mut history, (row - 1, col, Dir::Down));
                        to |= Dir::Up as u8;
                    }
                    Dir::Right => {
                        add(&mut rays, &mut history, (row, col - 1, Dir::Right));
                        to |= Dir::Left as u8;
                    }
                }
            } else if *cell == Cell::MirrorB {
                //  '\'
                match from {
                    Dir::Up => {
                        add(&mut rays, &mut history, (row, col + 1, Dir::Left));
                        to |= Dir::Right as u8;
                    }
                    Dir::Left => {
                        add(&mut rays, &mut history, (row + 1, col, Dir::Up));
                        to |= Dir::Down as u8;
                    }
                    Dir::Down => {
                        add(&mut rays, &mut history, (row, col - 1, Dir::Right));
                        to |= Dir::Left as u8;
                    }
                    Dir::Right => {
                        add(&mut rays, &mut history, (row - 1, col, Dir::Down));
                        to |= Dir::Up as u8;
                    }
                }
            } else if *cell == Cell::MirrorF {
                //  '/'
                match from {
                    Dir::Up => {
                        add(&mut rays, &mut history, (row, col - 1, Dir::Right));
                        to |= Dir::Left as u8;
                    }
                    Dir::Left => {
                        add(&mut rays, &mut history, (row - 1, col, Dir::Down));
                        to |= Dir::Up as u8;
                    }
                    Dir::Down => {
                        add(&mut rays, &mut history, (row, col + 1, Dir::Left));
                        to |= Dir::Right as u8;
                    }
                    Dir::Right => {
                        add(&mut rays, &mut history, (row + 1, col, Dir::Up));
                        to |= Dir::Down as u8;
                    }
                }
            } else if *cell == Cell::SplitH {
                //  '-'
                add(&mut rays, &mut history, (row, col - 1, Dir::Right));
                add(&mut rays, &mut history, (row, col + 1, Dir::Left));
                to |= Dir::Right as u8;
                to |= Dir::Left as u8;
            } else if *cell == Cell::SplitV {
                //  '|'
                add(&mut rays, &mut history, (row - 1, col, Dir::Down));
                add(&mut rays, &mut history, (row + 1, col, Dir::Up));
                to |= Dir::Down as u8;
                to |= Dir::Up as u8;
            }

            // if to == cell.2 {
            //     rays.pop();
            //     if cell.0 == Cell::SplitH || cell.0 == Cell::SplitV {
            //         rays.pop();
            //     }
            // }
            *cto = to;
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let mut map = parse(file);
        // println!("{}", _display(&map));
        process(&mut map, vec![(0, 0, Dir::Left)]);
        // println!("{}", _display(&map));
        map.iter().filter(|(_cell, active, _)| active != &0).count() as u64
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        // println!("{}", _display(&map));
        let mut starts = Vec::new();
        for col in 0..map.cols() {
            starts.push((0, col, Dir::Up));
            starts.push((map.rows() - 1, col, Dir::Down));
        }
        for row in 0..map.rows() {
            starts.push((row, 0, Dir::Left));
            starts.push((row, map.cols() - 1, Dir::Right));
        }

        let rank: Vec<u64> = starts
            .par_iter()
            .map(|start| {
                let mut temp_map = map.clone();
                process(
                    &mut temp_map,
                    vec![(start.0 as i32, start.1 as i32, start.2)],
                );
                temp_map
                    .iter()
                    .filter(|(_cell, active, _)| active != &0)
                    .count() as u64
            })
            .collect();

        *rank.iter().max().unwrap()
    }
}
