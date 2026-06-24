
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Transform {
    Non = 0,
    RotRight,
    RotHalf,
    RotLeft,
    FlipHor,
    FlipVir
}

pub fn parse(file: String) -> (HashMap<usize,Vec<Vec<bool>>>, Vec<((usize,usize),HashMap<usize, u32>)>) {
    let mut map_pattern = HashMap::new();
    let mut vec_yard = Vec::new();
    let re_pattern = Regex::new(
        r"(?:(\d+):\n((?:[.#]+\n)+))"
    ).unwrap();

    for ( _full, [ id, str ] ) in
        re_pattern.captures_iter(file.as_str())
        .map(|c| c.extract())
    {
        map_pattern.insert(id.parse().expect("cannot parse id"),
            str.lines().map(|line| {
                line.chars().filter_map(|c|
                    match c {
                        '#' => Some(true),
                        '.' => Some(false),
                        _ => None
                    }
                ).collect::<Vec<bool>>()
            }).collect::<Vec<Vec<bool>>>()
        );
    }

    let re_yard = Regex::new(
        r"(?:(\d+)x(\d+)\: ((?:\d+ *)+))"
    ).unwrap();
    for (_full, [width,height,amount]) in
        re_yard.captures_iter(file.as_str())
        .map(|c| c.extract()) {
        vec_yard.push((
            (
                width.parse().expect("cannot parse width"),
                height.parse().expect("cannot parse height"),
            ),
            amount.split(" ").enumerate()
                .map(|(id, a)| (id, a.parse().expect("cannot parse amount")))
                .collect()
        ));
    }

    (map_pattern, vec_yard)
}

pub fn put_yard (
    map: Vec<Vec<bool>>,
    pattern: &Vec<Vec<bool>>,
    target_x: usize,
    target_y: usize,
    direction: Transform
) -> Result<Vec<Vec<bool>>,String> {
    let size = pattern.len();
    let rotate_pattern = transform(pattern, direction);

    let mut new_map = map.clone();
    for y in 0..size {
        for x in 0..size {
            let cell = rotate_pattern.get(y).expect("cannot get cell by y {y}")
                              .get(x).expect("cannot get pattern by x {x}");
            let target = new_map.get_mut(target_y + y).expect("cannot access target y")
                              .get_mut(target_x + x).expect("cannot access target x");
            if *cell == true {
                if *target {
                    return Err("Cell {x},{y} is already place".to_string());
                } else {
                    *target = true;
                }
            }
        }
    }
    Ok(new_map)
}

pub fn display (map: &Vec<Vec<bool>>) -> String {
    ("\n".to_string() + map.iter().map(|line| {
        line.iter().map(|&cell| {
            match cell {
                true => "#",
                false => ".",
            }
        }).collect::<String>() + "\n"
    }).collect::<String>().as_str()).to_string()
}

pub fn transform (map: &Vec<Vec<bool>>, trans: Transform) -> Vec<Vec<bool>> {
    let size = map.len();
    if size == 0 {
        return Vec::new();
    }
    let mut new_map = vec![vec![false; size]; size];
    for new_y in 0..size {
        for new_x in 0..size {
            let (old_y, old_x) = match trans {
                Transform::Non => (new_y, new_x),
                Transform::RotLeft => (new_x, size - 1 - new_y),
                Transform::RotRight => (size - 1 - new_x, new_y),
                Transform::RotHalf => (size - 1 - new_y, size - 1 - new_x),
                Transform::FlipVir => (new_y, size - 1 - new_x),
                Transform::FlipHor => (size - 1 - new_y, new_x),
            };
            new_map[new_y][new_x] = map[old_y][old_x];
        }
    }
    new_map
}

pub fn generate_yard(width: usize, height: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width as usize]; height as usize ]
}

pub fn valid_put_yard(map: Vec<Vec<bool>>, patterns: &HashMap<usize, Vec<Vec<bool>>>, amounts: Vec<usize>) -> bool {
    if amounts.len() == 0 {
        println!("{}", display(&map));
        return true;
    }
    let &idx = amounts.first().expect("cannot get id from list");
    let pattern = patterns.get(&idx).expect("cannot get pattern");
    let size = pattern.len();
    let width = map.first().expect("cannot get first line").len();
    let height = map.len();
    for y in 0..=(height - size) {
        for x in 0..=(width - size) {
            for rot in [
                Transform::Non,
                Transform::RotLeft,
                Transform::RotRight,
                Transform::RotHalf,
                Transform::FlipHor,
                Transform::FlipVir
            ] {
                let result = put_yard(map.clone(), pattern, x, y, rot);
                if result.is_ok() {
                    let new_yard = result.unwrap();
                    let new_amounts: Vec<_> = amounts.clone().iter().skip(1).map(|p| p.clone()).collect();
                    // println!("remain {} , {}", new_pattern.len(), display(&new_yard));
                    if valid_put_yard(new_yard, patterns, new_amounts) {
                        return true;
                    }
                } else {
                    // println!("Test {x} {y} {rot:?}");
                }
            }
        }
    }
    false
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let (patterns, yards) =  crate::parse(file);
        let mut count = 0;
        for ((width, height), quota) in yards {
            let map = generate_yard(width, height);
            let pattern_set = quota.iter()
            .flat_map(|(&id, &amount)| {
                (0..amount).map(|_| {
                    id
                }).collect::<Vec<usize>>()
            }).collect::<Vec<usize>>();
            println!("{width}x{height} :");
            if valid_put_yard(map, &patterns, pattern_set) {
                count += 1;
            }
        }
        count
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
