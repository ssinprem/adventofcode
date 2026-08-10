use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub fn parse(file: String) -> Vec<(Dir, u32, String)> {
    file.lines()
    .filter_map(|line| {

        let regex = Regex::new(r"([ULRD]) (\d) \(([#a-f0-9]+)\)").unwrap();

        if let Some(iter) = regex.captures(line) &&
            let Some(dir) = iter.get(1) &&
            let Some(num) = iter.get(2) &&
            let Some(color) = iter.get(3)
        {
            Some((
                match dir.as_str() {
                    "R" => Dir::RIGHT,
                    "L" => Dir::LEFT,
                    "U" => Dir::UP,
                    "D" => Dir::DOWN,
                    _ => unreachable!()
                },
                num.as_str().parse::<u32>().unwrap(),
                color.as_str().to_string()
            ))
        } else {
            None
        }
    }).collect()
}

pub fn _display(hs: &HashSet<(i32,i32)>) {
    let min_rows = hs.iter().min_by_key(|(_c,r)| *r).unwrap().1;
    let min_cols = hs.iter().min_by_key(|(c,_r)| *c).unwrap().0;
    let max_rows = hs.iter().max_by_key(|(_c,r)| *r).unwrap().1;
    let max_cols = hs.iter().max_by_key(|(c,_r)| *c).unwrap().0;

    for row in min_rows..=max_rows {
        for col in min_cols..=max_cols {
            if hs.contains(&(col,row)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub mod part1 {
    use std::{collections::HashSet, println};

    use super::*;

    pub fn digger(seq: Vec<(Dir, u32, String)>) -> HashSet<(i32, i32)> {
        let mut hs = HashSet::new();

        let mut current = (0_i32,0_i32);
        hs.insert(current);

        for (dir,num, _color) in seq {
            for _n in 0..num {
                let diff = match dir {
                    Dir::UP => (0,-1),
                    Dir::DOWN => (0,1),
                    Dir::RIGHT => (1,0),
                    Dir::LEFT => (-1,0)
                };

                current = (current.0 + diff.0 , current.1 + diff.1);
                hs.insert(current);
            }
        }
        hs
    }

    pub fn fill(digged: &HashSet<(i32,i32)>) -> HashSet<(i32,i32)> {
        let min_rows = digged.iter().min_by_key(|(_c,r)| *r).unwrap().1;
        let min_cols = digged.iter().min_by_key(|(c,_r)| *c).unwrap().0;
        let max_rows = digged.iter().max_by_key(|(_c,r)| *r).unwrap().1;
        let max_cols = digged.iter().max_by_key(|(c,_r)| *c).unwrap().0;
        let mut hs = digged.clone();

        for row in min_rows..=max_rows {
            let mut edge = 0;
            let mut count = 0;
            for col in min_cols..=max_cols {
                
                let cur = digged.contains(&(col,row));
                let next  = digged.contains(&(col+1,row));
                if !cur && next {
                    // inside = false;
                } else if cur && !next {
                    edge += 1;
                }
                
                if edge % 2 == 1 {
                    count += 1;
                }
                if edge % 2 == 0 && count > 0 {
                    for i in 0..count {
                        hs.insert((col-i, row));
                    }
                    count = 0;
                }
            }
        }
        hs
    }

    pub fn solve(file: String) -> u64 {
        let seq = parse(file);

        let mut hs = digger(seq);
        _display(&hs);
        println!("============");
        hs = fill(&hs);
        _display(&hs);
        hs.len() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
