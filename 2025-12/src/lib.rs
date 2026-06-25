
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
pub enum Transform {
    Non = 0,
    RotRight,
    RotHalf,
    RotLeft,
    FlipHor,
    FlipVir,
    FlipInc,
    FlipDec,
}
impl Transform {
    fn get_members() -> Vec<Self> {
        vec![
            Self::Non,
            Self::RotRight,
            Self::RotHalf,
            Self::RotLeft,
            Self::FlipHor,
            Self::FlipVir,
            Self::FlipInc,
            Self::FlipDec,
        ]
    }
}

#[allow(clippy::type_complexity)]
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
    pattern: &[Vec<bool>],
    target_x: usize,
    target_y: usize
) -> Result<Vec<Vec<bool>>,String> {
    let size = pattern.len();
    let mut new_map = map.clone();
    for y in 0..size {
        for x in 0..size {
            let cell = pattern.get(y).expect("cannot get cell by y {y}")
                              .get(x).expect("cannot get pattern by x {x}");
            let target = new_map.get_mut(target_y + y).expect("cannot access target y")
                              .get_mut(target_x + x).expect("cannot access target x");
            if *cell {
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

pub fn display (map: &[Vec<bool>]) -> String {
    ("\n".to_string() + map.iter().map(|line| {
        line.iter().map(|&cell| {
            match cell {
                true => "#",
                false => ".",
            }
        }).collect::<String>() + "\n"
    }).collect::<String>().as_str()).to_string()
}

#[allow(clippy::needless_range_loop)]
pub fn transform (pattern: &[Vec<bool>], trans: Transform) -> Vec<Vec<bool>> {
    let size = pattern.len();
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
                Transform::FlipInc => (size - 1  - new_x, size - 1 - new_y),
                Transform::FlipDec => (new_x, new_y),
            };
            new_map[new_y][new_x] = pattern[old_y][old_x];
        }
    }
    new_map
}

pub fn generate_yard(width: usize, height: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; height ]
}

pub fn get_varints(pattern: &[Vec<bool>]) -> Vec<Vec<Vec<bool>>> {
    let mut set = HashSet::new();
    for rot in Transform::get_members() {
        set.insert(transform(pattern, rot));
    }
    set.iter().cloned().collect()
}

pub fn valid_put_yard(
    map: Vec<Vec<bool>>,
    patterns_variants: &HashMap<usize,
    Vec<Vec<Vec<bool>>>>,
    amounts: Vec<usize>
) -> bool {
    if amounts.is_empty() {
        println!("{}", display(&map));
        return true;
    }
    let &idx = amounts.first().expect("cannot get id from list");
    let pattern_variants = patterns_variants.get(&idx).expect("cannot get pattern");
    let size = pattern_variants.first().expect("cannot get pattern size").len();
    let width = map.first().expect("cannot get first line").len();
    let height = map.len();
    for pattern_trans in pattern_variants {
        for y in 0..=(height - size) {
            for x in 0..=(width - size) {
                
                let result = put_yard(map.clone(), pattern_trans, x, y);
                if let Ok(new_yard) = result {
                    let new_amounts: Vec<_> = amounts.clone().iter().skip(1).copied().collect();
                    // println!("remain {} {:?} , {}", new_amounts.len(), new_amounts, display(&new_yard));
                    if valid_put_yard(new_yard, patterns_variants, new_amounts) {
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
        let patterns_variant: HashMap<usize, Vec<Vec<Vec<bool>>>> = patterns.iter()
            .map(|(idx, pat)| {
                (*idx, get_varints(pat))
            }).collect();
        
        // for (idx, variants) in &patterns_variant {
        //     print!("{idx}: [{}]", variants.len());
        //     for v in variants {
        //         print!("{}", display(&v));
        //     }
        // }

        for (idx, ((width, height), quota)) in yards.iter().enumerate() {
            let map = generate_yard(*width, *height);
            let pattern_set = quota.iter()
            .flat_map(|(&id, &amount)| {
                (0..amount).map(|_| {
                    id
                }).collect::<Vec<usize>>()
            }).collect::<Vec<usize>>();
            // calculate all part area
            let part_area = pattern_set.iter().map(|idx| {
                patterns.get(idx).expect("Cannot get pattarn by id")
                    .iter().map(|line| {
                        line.iter().filter(|&b| *b)
                        .count()
                    }).sum::<usize>()
            }).sum::<usize>();

            println!("{idx}. {width}x{height} :");
            if part_area > width*height {
                println!("   oversize {part_area} > {}", width*height);
            } else if valid_put_yard(map, &patterns_variant, pattern_set) {
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
