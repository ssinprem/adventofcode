#[derive(PartialEq,Clone, Debug)]
pub enum STATE {
    BEAM,
    SPITTER,
    SPITTERX,
    NONE
}

use std::collections::{HashMap, HashSet};
pub fn print_map(map : HashMap<(usize, usize), STATE>, set: HashSet<(usize, usize)>) {
    let max = map.iter().max_by_key(|((x,y),_v)| x+y ).unwrap().0;
    for y in 0..max.1 {
        for x in 0..max.0 {
            print!("{}",
                match (set.get(&(x,y)), map.get(&(x,y))) {
                    (Some(_), _)                => '#',
                    (_, Some(STATE::BEAM))      => '|',
                    (_, Some(STATE::SPITTER))   => '^',
                    (_, Some(STATE::SPITTERX))  => 'x',
                    (_, Some(STATE::NONE))      => '.',
                    (_, _ )                     => '?'
                }
            )
        }
        println!();
    }
}

pub mod part1 {
    use std::collections::{HashMap, HashSet};
    use crate::STATE::{BEAM, SPITTERX, SPITTER, NONE};

    pub fn solve(file: String) -> u64 {
        let mut map = HashMap::new();
        let mut count = 0;
        let height = file.lines().count();
        let width = file.find("\n").unwrap();
        for (y,line) in file.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                map.insert((x,y), match char {
                    'S' => BEAM,
                    '^' => SPITTER,
                    _ => NONE
                });
            }
        }
        for y in 1..height {
            for x in 0..width {
                match (map.get(&(x,y)), map.get(&(x,y-1))) {
                    (Some(NONE), Some(BEAM))    => {
                        map.insert((x,y), BEAM);
                    },
                    (Some(SPITTER), Some(BEAM)) => {
                        count += 1;
                        map.insert((x,y), SPITTERX);
                        map.insert((x-1, y), BEAM);
                        map.insert((x+1,y), BEAM);
                    },
                    (_ , _) => {}
                }
            }
        }
        crate::print_map(map, HashSet::new());
        count
    }
}

pub mod part2 {
    use std::collections::HashMap;
    use crate::{STATE::{BEAM, NONE, SPITTER}, print_map};

    pub fn solve(file: String) -> u64 {
        let mut count = 0;
        let mut map = HashMap::new();
        let mut currs = Vec::<(usize,usize)>::new();
        let height = file.lines().count();
        for (y,line) in file.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                map.insert((x,y), match char {
                    'S' => { currs.push((x,y+1)); BEAM },
                    '^' => SPITTER,
                    _ => NONE
                });
            }
        }
        while let Some(curr) = currs.pop() {
            if curr.1 >= height {
                count += 1;
                continue;
            }

            match map.get(&curr) {
                Some(NONE) => currs.push((curr.0,curr.1+1)),
                Some(SPITTER) => {
                    if curr.0 != 0 {
                        currs.push((curr.0-1,curr.1+1));
                    }
                    currs.push((curr.0+1,curr.1+1));
                },
                None => { },
                _ => { unreachable!() }
            }
            if count % 10000000 == 0 {
                println!("end {count} , remain {},  current {:?}", currs.len(), curr);
                print_map(map.clone(), currs.iter().copied().collect());
            }
        }
        println!("end {count} , remain {}", currs.len());
        count
    }
}
