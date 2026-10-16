use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<(isize, isize), u8> {
    let mut virus = HashMap::new();

    let rows = file.lines().count();
    let cols = file.lines().next().unwrap().chars().count();

    file.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '#' {
                virus.insert((
                    (x as isize) - (cols / 2) as isize,
                    (y as isize) - (rows / 2) as isize,
                ), 1);
            }
        });
    });

    virus
}

pub enum Turn {
    Right,
    Left,
    Reverse
}
pub fn turn(dir: (isize, isize), turn: Turn) -> (isize, isize) {
    match (turn, dir) {
        (Turn::Right, (0, -1)) => (1, 0),
        (Turn::Right, (1, 0)) => (0, 1),
        (Turn::Right, (0, 1)) => (-1, 0),
        (Turn::Right, (-1, 0)) => (0, -1),
        (Turn::Left, (0, -1)) => (-1, 0),
        (Turn::Left, (-1, 0)) => (0, 1),
        (Turn::Left, (0, 1)) => (1, 0),
        (Turn::Left, (1, 0)) => (0, -1),
        (Turn::Reverse, (0, -1)) => (0, 1),
        (Turn::Reverse, (-1, 0)) => (1, 0),
        (Turn::Reverse, (0, 1)) => (0, -1),
        (Turn::Reverse, (1, 0)) => (-1, 0),
        _ => unreachable!(),
    }
}

pub fn _display(virus: &HashMap<(isize, isize), u8>, pos: (isize, isize), dir: (isize, isize)) {
    // let min_x = virus.iter().map(|(x,_)|*x).min().unwrap();
    // let max_x = virus.iter().map(|(x,_)|*x).max().unwrap();
    // let min_y = virus.iter().map(|(_,y)|*y).min().unwrap();
    // let max_y = virus.iter().map(|(_,y)|*y).max().unwrap();

    let min_x = -12; //min_x.min(pos.0);
    let max_x = 12; //max_x.max(pos.0);
    let min_y = -12; //min_y.min(pos.1);
    let max_y = 12; //max_y.max(pos.1);

    println!(
        "direction = {}",
        match dir {
            (0, 1) => "v",
            (0, -1) => "^",
            (1, 0) => ">",
            (-1, 0) => "<",
            _ => unreachable!(),
        }
    );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if pos.0 == x && pos.1 == y {
                print!("[")
            } else {
                print!(" ")
            }
            if let Some(v) = virus.get(&(x, y)) {
                match v {
                    1 => print!("W"),
                    2 => print!("I"),
                    3 => print!("F"),
                    _ => print!("?")
                }
            } else {
                print!(".")
            }
            if pos.0 == x && pos.1 == y {
                print!("]")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}

pub mod part1 {

    use crate::{parse, turn, Turn};

    pub fn solve(file: String) -> usize {
        let mut virus = parse(file);
        let mut pos = (0, 0); // center
        let mut dir = (0, -1); // north first

        let mut count = 0;

        for _ in 0..10000 {
            // check current pos
            if let Some(_status) = virus.get(&pos) {
                // infected -> turn right, clean
                dir = turn(dir, Turn::Right);
                virus.remove(&pos);
            } else {
                // clean -> turn left, infected
                dir = turn(dir, Turn::Left);
                virus.insert(pos, 1);
                count += 1;
            }
            // walk
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }

        count
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
