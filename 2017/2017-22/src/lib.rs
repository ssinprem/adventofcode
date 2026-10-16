use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<(isize, isize), State> {
    let mut virus = HashMap::new();

    let rows = file.lines().count();
    let cols = file.lines().next().unwrap().chars().count();

    file.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '#' {
                virus.insert(
                    (
                        (x as isize) - (cols / 2) as isize,
                        (y as isize) - (rows / 2) as isize,
                    ),
                    State::Infected,
                );
            }
        });
    });

    virus
}

pub enum Turn {
    Right,
    Left,
    Reverse,
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

#[derive(PartialEq)]
pub enum State {
    Weakened,
    Flagged,
    Infected
}
pub fn _display(virus: &HashMap<(isize, isize), State>, pos: (isize, isize), dir: (isize, isize)) {
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
                    State::Weakened => print!("W"),
                    State::Infected => print!("#"),
                    State::Flagged => print!("F"),
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

    use crate::{Turn, State, parse, turn};

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
                virus.insert(pos, State::Infected);
                count += 1;
            }
            // walk
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }

        count
    }
}

pub mod part2 {
    use crate::{Turn, State, parse, turn};

    pub fn solve(file: String) -> usize {
        let mut virus = parse(file);
        let mut pos = (0, 0); // center
        let mut dir = (0, -1); // north first

        let mut count = 0;

        for _ in 0..10_000_000 {
            // check current pos
            if let Some(status) = virus.get_mut(&pos) {
                if *status == State::Flagged {
                    // flagged -> turn reverse, clean
                    dir = turn(dir, Turn::Reverse);
                    virus.remove(&pos);
                } else if *status == State::Infected {
                    // infected -> turn right, flagged
                    dir = turn(dir, Turn::Right);
                    virus.insert(pos, State::Flagged);
                } else if *status == State::Weakened {
                    // weakened -> not turn, infected
                    virus.insert(pos, State::Infected);
                    count += 1;
                }
            } else {
                // clean -> turn left, weakened
                dir = turn(dir, Turn::Left);
                virus.insert(pos, State::Weakened);
            }
            // walk
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }

        count
    }
}
