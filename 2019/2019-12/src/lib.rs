#[derive(Clone, Debug, PartialEq)]
pub struct Moon {
    pos: (isize, isize, isize),
    vec: (isize, isize, isize),
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
            * (self.vec.0.abs() + self.vec.1.abs() + self.vec.2.abs()) as usize
    }
}

pub fn parse(file: String) -> Vec<Moon> {
    let regex = regex::Regex::new(r"<x=([-0-9]+), y=([-0-9]+), z=([-0-9]+)").unwrap();

    file.lines()
        .map(|line| {
            let cap = regex.captures(line).unwrap();
            let x = cap.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<isize>().unwrap();
            let z = cap.get(3).unwrap().as_str().parse::<isize>().unwrap();

            Moon {
                pos: (x, y, z),
                vec: (0, 0, 0),
            }
        })
        .collect()
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
    use utility::lcm;

    use crate::parse;

    pub fn solve(file: String) -> usize {
        let mut moons = parse(file);
        let original = moons.clone();

        let mut repeat = (0, 0, 0);
        for i in 0.. {
            if i != 0 {
                if repeat.0 == 0
                    && moons
                        .iter()
                        .zip(original.clone())
                        .all(|(m, o)| m.pos.0 == o.pos.0 && m.vec.0 == o.vec.0)
                {
                    repeat.0 = i
                }

                if repeat.1 == 0
                    && moons
                        .iter()
                        .zip(original.clone())
                        .all(|(m, o)| m.pos.1 == o.pos.1 && m.vec.1 == o.vec.1)
                {
                    repeat.1 = i
                }

                if repeat.2 == 0
                    && moons
                        .iter()
                        .zip(original.clone())
                        .all(|(m, o)| m.pos.2 == o.pos.2 && m.vec.2 == o.vec.2)
                {
                    repeat.2 = i
                }
            }
            if repeat.0 != 0 && repeat.1 != 0 && repeat.2 != 0 {
                break;
            }

            let temp = moons.clone();
            for moon in moons.iter_mut() {
                moon.update_vec(&temp);
            }
            for moon in moons.iter_mut() {
                moon.update_pos();
            }
        }

        lcm(repeat.0, lcm(repeat.1, repeat.2))
    }
}
