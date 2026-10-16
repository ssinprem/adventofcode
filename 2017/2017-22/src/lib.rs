use std::collections::HashSet;

pub fn parse(file: String) -> HashSet<(isize, isize)> {
    let mut virus = HashSet::new();

    let rows = file.lines().count();
    let cols = file.lines().next().unwrap().chars().count();

    file.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '#' {
                virus.insert((
                    (x as isize) - (cols / 2) as isize,
                    (y as isize) - (rows / 2) as isize,
                ));
            }
        });
    });

    virus
}

pub fn turn(dir: (isize, isize), right: bool) -> (isize, isize) {
    match (right, dir) {
        (true, (0, -1)) => (1, 0),
        (true, (1, 0)) => (0, 1),
        (true, (0, 1)) => (-1, 0),
        (true, (-1, 0)) => (0, -1),
        (false, (0, -1)) => (-1, 0),
        (false, (-1, 0)) => (0, 1),
        (false, (0, 1)) => (1, 0),
        (false, (1, 0)) => (0, -1),
        _ => unreachable!(),
    }
}

pub fn _display(virus: &HashSet<(isize, isize)>, pos: (isize, isize), dir: (isize, isize)) {
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
            if virus.contains(&(x, y)) {
                print!("#")
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
    use std::collections::HashSet;

    use crate::{parse, turn};

    pub fn solve(file: String) -> usize {
        let mut virus = parse(file);
        let mut pos = (0, 0); // center
        let mut dir = (0, -1); // north first

        let mut count = 0;
        println!("{virus:?}");

        for _ in 0..10000 {
            // check current pos
            let status = virus.contains(&pos);
            if status {
                // infected -> turn right, clean
                dir = turn(dir, true);
                virus.remove(&pos);
            } else {
                // clean -> turn left, infected
                dir = turn(dir, false);
                virus.insert(pos);
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
