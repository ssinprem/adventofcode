use grid::Grid;

#[derive(Default, Debug, PartialEq)]
pub enum Cell {
    #[default]
    None,
    Space,
    Wall,
    Num(u32)
}

pub fn parse(file: String) -> Grid<Cell> {
    let mut grid = Grid::new(0,0);
    file.lines().filter(|line| !line.is_empty())
    .for_each(|line|
        grid.push_row(
            line.chars().map(|c| {
                match c {
                    '.' => Cell::Space,
                    '#' => Cell::Wall,
                    c if let Some(n) = c.to_digit(10)
                        => Cell::Num(n),
                    _ => unreachable!()
                }
            }).collect()
        )
    );
    grid
}

pub mod part1 {
    use std::collections::VecDeque;

use crate::{Cell::{Num, Wall, Space}, parse};

    pub fn solve(file: String) -> usize {
        let grid = parse(file);
        let mut paths = VecDeque::new();
        let mut shortest = usize::MAX;
        let start = grid.iter_rows()
        .enumerate().find_map(|(y, rows)| {
            rows.enumerate().find_map(|(x, cell)| {
                if *cell == Num(0) {
                    Some((y,x))
                } else {
                    None
                }
            })
        }).unwrap();
        let targets = grid.iter_rows().enumerate()
        .flat_map(|(y, rows)| {
            rows.enumerate().filter_map(|(x,cell)| {
                if *cell != Wall && *cell != Space {
                    Some((y,x))
                } else {
                    None
                }
            }).collect::<Vec<(usize,usize)>>()
        }).collect::<Vec<(usize,usize)>>();
        paths.push_back(vec![start]);

        while let Some(path) = paths.pop_back() {
            let len = path.len();
            println!("{len} {path:?}");
            if targets.iter().all(|target| path.contains(target)) {
                if len < shortest {
                    shortest = len;
                    println!("shortest {shortest}");
                }
            }
            if len >= shortest {
                continue;
            }

            let last = path.last().unwrap();
            [(-1,0),(1,0),(0,-1),(0,1)]
                .into_iter().for_each(|(dy,dx)| {
                    let next = (
                        (dy + last.0 as isize) as usize,
                        (dx + last.1 as isize) as usize
                    );
                    if path.iter().filter(|c| **c==next).count() > 3 {
                        return
                    }
                    if let Some(cell) = grid.get(next.0, next.1) {
                        if *cell != Wall {
                            let mut new_path = path.clone();
                            new_path.push(next);
                            paths.push_back(new_path);
                        }
                    }
            })
        }
        shortest - 1
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
