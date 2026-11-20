#[derive(Clone, Debug)]
pub struct Moon
{
    pos: (isize,isize,isize),
    vec: (isize,isize,isize)
}

impl Moon {
    fn update_vec(&mut self, other_moon: &[Moon]) {
        let mut vx = 0;
        let mut vy = 0;
        let mut vz = 0;

        for moon in other_moon {
            if moon.pos.0 > self.pos.0 {
                vx += 1;
            } else if moon.pos.0 < self.pos.0 {
                vx -= 1;
            }
            if moon.pos.1 > self.pos.1 {
                vy += 1;
            } else if moon.pos.1 < self.pos.1 {
                vy -= 1;
            }
            if moon.pos.2 > self.pos.2 {
                vz += 1;
            } else if moon.pos.2 < self.pos.2 {
                vz -= 1;
            }
        }
        self.vec.0 += vx;
        self.vec.1 += vy;
        self.vec.2 += vz;
    }

    fn update_pos(&mut self) {
        self.pos.0 += self.vec.0;
        self.pos.1 += self.vec.1;
        self.pos.2 += self.vec.2;
    }

    fn get_value(&self) -> usize {
        (self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()) as usize
        *
        (self.vec.0.abs() + self.vec.1.abs() + self.vec.2.abs()) as usize
    }
}

pub fn parse(file: String) -> Vec<Moon> {
    let regex = regex::Regex::new(r"<x=([-0-9]+), y=([-0-9]+), z=([-0-9]+)").unwrap();

    file.lines().map(|line| {
        let cap = regex.captures(line).unwrap();
        let x = cap.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<isize>().unwrap();
        let z = cap.get(3).unwrap().as_str().parse::<isize>().unwrap();

        Moon {
            pos: (x,y,z),
            vec: (0,0,0)
        }
    }).collect()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String, steps: usize) -> usize {
        let mut moons = parse(file);

        for _ in 0..steps {
            let temp = moons.clone();
            for moon in moons.iter_mut() {
                moon.update_vec(&temp);
            }
            for moon in moons.iter_mut() {
                moon.update_pos();
            }
        }

        moons.iter().map(|moon| moon.get_value()).sum()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
