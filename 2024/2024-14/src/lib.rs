use regex::*;

#[derive(Debug, Copy, Clone)]
pub struct Robot {
    pub pos: (i32,i32),
    pub vec: (i32,i32),
    pub limit: (i32,i32)
}

impl Robot {
    pub fn new(line: String, limit: (i32,i32)) -> Self {
        let reg = Regex::new(r"p=(-{0,1}\d+),(-{0,1}\d+) v=(-{0,1}\d+),(-{0,1}\d+)").unwrap();
        if let Some(cap) = reg.captures(&line) 
            && let Some(px) = cap.get(1)
            && let Some(py) = cap.get(2)
            && let Some(vx) = cap.get(3)
            && let Some(vy) = cap.get(4)
        {
            return Self {
                pos: (
                    px.as_str().parse().expect("cannot parse"),
                    py.as_str().parse().expect("cannot parse"),
                ),
                vec: (
                    vx.as_str().parse().expect("cannot parse"),
                    vy.as_str().parse().expect("cannot parse"),
                ),
                limit
            }
        }
        panic!("Cannot create Robot from '{line}'");
    }

    pub fn moved(&mut self) {
        self.pos.0 = self.pos.0 + self.vec.0;
        self.pos.1 = self.pos.1 + self.vec.1;
        if self.pos.0 < 0 {
            self.pos.0 += self.limit.0;
        }
        if self.pos.0 >= self.limit.0 {
            self.pos.0 -= self.limit.0;
        }
        if self.pos.1 < 0 {
            self.pos.1 += self.limit.1;
        }
        if self.pos.1 >= self.limit.1 {
            self.pos.1 -= self.limit.1;
        }
    }

    //    4 | 1
    //    --+--
    //    3 | 2
    pub fn get_quadrant(&self) -> usize {
        let midx = self.limit.0 / 2;
        let midy = self.limit.1 / 2;
        match (self.pos.0, self.pos.1) {
            (x,y) if x > midx && y < midy => 1,
            (x,y) if x > midx && y > midy => 2,
            (x,y) if x < midx && y > midy => 3,
            (x,y) if x < midx && y < midy => 4,
            _ => 0
        }
    }

    
}

pub fn print_map(robots: &[Robot], limit: (i32,i32)) -> String {
    let midx = limit.0 / 2;
    let midy = limit.1 / 2;
    let mut str = String::new();
    
    (0..limit.1).for_each(|y| {
        (0..limit.0).for_each(|x|{
            let n = robots.iter()
                .filter(|robot| robot.pos == (x,y)).count();
            if n>0 {
                str += format!("{n}").as_str();
            } else if x == midx && y == midy {
                str += "+";
            } else if x == midx {
                str += "|";
            } else if y == midy {
                str += "—";
            } else {
                str += ".";
            }
        });
        str += "\n";
    });
    str
}

pub fn parse(file: String, limit: (i32,i32)) -> Vec<Robot> {
    file.lines().filter_map(|line| {
        if line.is_empty() {
            None
        } else {
            Some(Robot::new(line.to_string(), limit))
        }
    }).collect()
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String, limit: (i32,i32)) -> u64 {
        let mut robots = parse(file, limit);

        println!("{}", print_map(robots.as_slice(), limit));
        (0..100).for_each(|_rep| {
            robots.iter_mut().for_each(|robot| {
                robot.moved();
            });
        });
        println!("{}", print_map(robots.as_slice(), limit));

        (1..=4).map(|quadrant| {
            robots.iter().filter(
                |robot| robot.get_quadrant() == quadrant
            ).count()
        }).inspect(|p| println!("{:?}",p))
        .product::<usize>() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
