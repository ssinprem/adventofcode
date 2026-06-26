use grid::*;

use crate::State::{Blank, Obstruct, Place};

#[derive(Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Blank, 
    Obstruct,
    Place
}

pub fn parse(file: String) -> (Grid<State>,(i32,i32)) {
    let mut current = (0,0);
    let cols = file.lines()
        .find(|line| !line.is_empty()).expect("cannot get first line")
        .chars().count();
    let mut grid= Grid::<State>::new(0,cols);
    file.lines().filter(|line| !line.is_empty())
        .enumerate().for_each(|(y,line)| {
            grid.insert_row(y,
                line.chars().enumerate().map(|(x,char)| {
                    match char {
                        '.' => Blank,
                        '#' => Obstruct,
                        '^' => {
                            current = (x as i32,y as i32);
                            Place
                        },
                        _ => unreachable!("Invalid char")
                    }
                }).collect()
            );
        });

    (grid, current)
}

pub fn moved(current: (i32,i32),diff: (i32,i32)) -> (i32,i32) {
    (
        current.0 + diff.0,
        current.1 + diff.1
    )
}

pub mod part1 {
    use super::*;

    const DIRECTION : [(i32,i32); 4] = [
        ( 0,-1), // N
        ( 1, 0), // E
        ( 0, 1), // S
        (-1, 0)  // W
    ];

    fn _print_map(grid: &Grid<State>, current: &(i32, i32), dir: usize ) {
        for y in 0..grid.rows() as i32 {
            for x in 0..grid.cols() as i32 {
                if current == &(x,y) {
                    match dir {
                        0 => print!("^"),
                        1 => print!(">"),
                        2 => print!("v"),
                        3 => print!("<"),
                        _ => {}
                    }
                } else {
                    match grid.get(x,y) {
                        Some(Obstruct) => print!("#"),
                        Some(Blank) => print!("."),
                        Some(Place) => print!("X"),
                        _ => {}
                    }
                }
            }
            println!()
        }
    }

    pub fn solve(file: String) -> u64 {
        let (mut grid, mut current) = parse(file);
        let mut count = 0;
        let mut dir = 0;
        loop {
            let next = moved(current, DIRECTION[dir]);
            if let Some(cell) = grid.get(next.1, next.0) {
                if cell == &Obstruct {
                    dir = (dir+1) % 4;
                    println!("{count:5} -- {next:?} found obstruct change {dir}");
                } else {
                    println!("{count:5} -- {next:?}");
                    let current_cell = grid.get_mut(current.1, current.0)
                        .expect("Cannot modify current cell");
                    *current_cell = Place;
                    current = next;
                    count += 1;
                }
                // _print_map(&grid, &current, dir);
            } else {
                return grid.iter().filter(|&state| state == &Place).count() as u64 + 1;
            }
        }
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
