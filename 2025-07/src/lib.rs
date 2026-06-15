#[derive(PartialEq)]
pub enum STATE {
    BEAM,
    SPITTER,
    NONE
}

pub mod part1 {
    use std::collections::HashMap;
    use crate::STATE::{self, BEAM, SPITTER, NONE};

    pub fn solve(file: String) -> u64 {
        let mut map = HashMap::<(usize,usize), STATE>::new();
        let mut count = 0;
        file.lines().enumerate()
        .for_each(|(y, line)| {
            print!("{y:<4}");
            line.chars().enumerate()
            .for_each(|(x,char)| {
                if y == 0 {
                    if char == 'S' {
                        map.insert((y,x), BEAM);
                        print!("S");
                    } else {
                        map.insert((y,x), NONE);
                        print!(".");
                    }
                } else {
                    if map.get(&(y,x)).is_some(){
                        // skip if it already apply
                        print!(".");
                    } else if char == '.' // top is beam follow through
                        && map.get(&(y-1,x)).is_some()
                        && map.get(&(y-1,x)) == Some(&BEAM)
                    {
                        map.insert((y,x), BEAM);
                        print!("|");
                    } else if char == '^' // top is beam, split it.
                        && map.get(&(y-1,x)).is_some()
                        && map.get(&(y-1,x)) == Some(&BEAM)
                    {
                        map.insert((y,x), SPITTER);
                        map.insert((y,x-1), BEAM);
                        map.insert((y,x+1), BEAM);
                        count += 1;
                        print!("^");
                    } else if char == '^' {
                        map.insert((y,x), SPITTER);
                        print!("x");
                    } else {
                        map.insert((y,x), NONE);
                        print!(".");
                    }
                }
            });
            println!();
        });
        count
    }
}

pub mod part2 {
    pub fn solve(file: String) -> u64 {
        0
    }
}
