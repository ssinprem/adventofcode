#[derive(PartialEq,Clone, Debug)]
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
    use std::collections::HashMap;
    use crate::STATE::{self, BEAM, SPITTER, NONE};

    fn start(map :HashMap<(usize,usize), STATE>, mut curr: (usize,usize), height: usize) -> u64 {
        while curr.1 < height {
            match map.get(&curr) {
                Some(NONE) | Some(BEAM) =>  { curr.1 += 1 },
                Some(SPITTER) =>  {
                    return start(map.clone(), (curr.0-1, curr.1), height) +
                           start(map.clone(), (curr.0+1, curr.1), height);
                }
                _ =>  { return 0; }
            }
        }
        // print!("!");
        // println!("end at {curr:?}");
        1
    }

    pub fn solve(file: String) -> u64 {
        let mut map = HashMap::new();
        let mut curr = (0,0);
        let height = file.lines().count();
        for (y,line) in file.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                map.insert((x,y), match char {
                    'S' => { curr = (x,y); BEAM },
                    '^' => SPITTER,
                    _ => NONE
                });
            }
        }
        // Start with 'S'
        start(map.clone(), curr, height)
    }
}
