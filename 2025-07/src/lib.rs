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

    fn split(mut map :HashMap<(usize,usize), STATE>, curr: (usize,usize)) -> u64 {
        let w = map.keys().max_by_key(|(x,_y)| x ).unwrap().0;
        let h = map.keys().max_by_key(|(_x,y)| y ).unwrap().1;
        let mut x = curr.0;
        let mut y = curr.1;
        while y < h {
            while x < w {
                let cell = map.get(&(x,y)).unwrap();
                if cell == &NONE {
                    if map.get(&(x,y-1)) == Some(&BEAM) {
                        map.insert((x,y), BEAM);
                    }
                } else if cell == &SPITTER {
                    if map.get(&(x,y-1)) == Some(&BEAM) {
                        let mut map_l = map.clone();
                        let mut map_r = map.clone();
                        map_l.insert((x-1,y), BEAM);
                        map_r.insert((x+1,y), BEAM);
                        return split(map_l.clone(), (x+2,y)) +
                               split(map_r.clone(), (x+2,y))
                    }
                }
                x += 1;
            }
            x = 0;
            y += 1;
        }
        print!("!");
        1
    }

    pub fn solve(file: String) -> u64 {
        let mut map = HashMap::new();
        for (y,line) in file.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                map.insert((x,y), match char {
                    'S' => BEAM,
                    '^' => SPITTER,
                    _ => NONE
                });
            }
        }
        split(map.clone(), (0,1))
    }
}
