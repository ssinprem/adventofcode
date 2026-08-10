use regex::Regex;

#[derive(Debug)]
pub enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn parse(file: String) -> Vec<(Dir, i64, String)> {
    file.lines()
        .filter_map(|line| {
            let regex = Regex::new(r"([ULRD]) (\d+) \((#[a-f0-9]+)\)").unwrap();
            let caps = regex.captures(line)?;
            let dir = match caps.get(1)?.as_str() {
                "R" => Dir::RIGHT,
                "L" => Dir::LEFT,
                "U" => Dir::UP,
                "D" => Dir::DOWN,
                _ => return None,
            };
            let num = caps.get(2)?.as_str().parse::<i64>().ok()?;
            let color = caps.get(3)?.as_str().to_string();
            Some((dir, num, color))
        })
        .collect()
}

pub fn area(instructions: Vec<(Dir, i64, String)>) -> u64 {
    let mut vertices = vec![(0, 0)];
    let mut perimeter = 0_i64;

    for (dir, dist, _) in instructions {
        let (last_x, last_y) = *vertices.last().unwrap();
        let (dx, dy) = match dir {
            Dir::UP => (0, -dist),
            Dir::DOWN => (0, dist),
            Dir::LEFT => (-dist, 0),
            Dir::RIGHT => (dist, 0),
        };
        vertices.push((last_x + dx, last_y + dy));
        perimeter += dist;
    }

    // Shoelace formula
    let mut area = 0_i64;
    for i in 0..vertices.len() - 1 {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];
        area += x1 * y2 - x2 * y1;
    }
    area = area.abs() / 2;

    // Pick's Theorem: A = I + B/2 - 1
    // We want I + B, which is the total number of points.
    // I = A - B/2 + 1
    // Total = I + B = A - B/2 + 1 + B = A + B/2 + 1
    (area + perimeter / 2 + 1) as u64
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let instructions = parse(file);

        area(instructions)
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let mut instructions = parse(file);

        instructions = instructions
            .iter()
            .map(|(_dir, _dist, str)| {
                let str = str.strip_prefix("#").unwrap();

                let (text, last) = str.split_at(5);
                let dir = match last {
                    "0" => Dir::RIGHT,
                    "1" => Dir::DOWN,
                    "2" => Dir::LEFT,
                    "3" => Dir::UP,
                    _ => unreachable!(),
                };
                let dist = u32::from_str_radix(text, 16).unwrap();
                (dir, dist as i64, str.to_string())
            })
            .collect();

        area(instructions)
    }
}
