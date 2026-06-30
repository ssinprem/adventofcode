#[derive(PartialEq, Clone, Debug)]
pub enum STATE {
    BEAM,
    SPITTER,
    SPITTERX,
    NONE,
}

use std::collections::{HashMap, HashSet};
pub fn print_map(map: HashMap<(usize, usize), STATE>, set: HashSet<(usize, usize)>) {
    let max = map.iter().max_by_key(|((x, y), _v)| x + y).unwrap().0;
    for y in 0..max.1 {
        for x in 0..max.0 {
            print!(
                "{}",
                match (set.get(&(x, y)), map.get(&(x, y))) {
                    (Some(_), _) => '#',
                    (_, Some(STATE::BEAM)) => '|',
                    (_, Some(STATE::SPITTER)) => '^',
                    (_, Some(STATE::SPITTERX)) => 'x',
                    (_, Some(STATE::NONE)) => '.',
                    (_, _) => '?',
                }
            )
        }
        println!();
    }
}

pub mod part1 {
    use crate::STATE::{BEAM, NONE, SPITTER, SPITTERX};
    use std::collections::{HashMap, HashSet};

    pub fn solve(file: String) -> u64 {
        let mut map = HashMap::new();
        let mut count = 0;
        let height = file.lines().count();
        let width = file.find("\n").unwrap();
        for (y, line) in file.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                map.insert(
                    (x, y),
                    match char {
                        'S' => BEAM,
                        '^' => SPITTER,
                        _ => NONE,
                    },
                );
            }
        }
        for y in 1..height {
            for x in 0..width {
                match (map.get(&(x, y)), map.get(&(x, y - 1))) {
                    (Some(NONE), Some(BEAM)) => {
                        map.insert((x, y), BEAM);
                    }
                    (Some(SPITTER), Some(BEAM)) => {
                        count += 1;
                        map.insert((x, y), SPITTERX);
                        map.insert((x - 1, y), BEAM);
                        map.insert((x + 1, y), BEAM);
                    }
                    (_, _) => {}
                }
            }
        }
        crate::print_map(map, HashSet::new());
        count
    }
}

pub mod part2 {
    use std::collections::HashMap;

    pub fn solve(file: String) -> u64 {
        // the map with col position and timelines used this cell
        let mut timelines = HashMap::<usize, u64>::new();
        let mut count = 0;
        let grid: Vec<&str> = file.lines().filter(|l| !l.is_empty()).collect();
        let width = grid[0].len();

        let s_col = grid[0].find('S').expect("Cannot find 'S'");
        timelines.insert(s_col, 1);
        for line in grid.iter().skip(1) {
            for (x, var) in timelines.clone() {
                match line.chars().nth(x) {
                    // follow through the beam
                    Some('.') => {}
                    // remove below and split to adjectcent
                    // in case touch the side end timeline, add the counter
                    Some('^') => {
                        timelines.remove(&x);
                        if x > 0 {
                            timelines
                                .entry(x - 1)
                                .and_modify(|v| {
                                    *v += var;
                                })
                                .or_insert(var);
                        } else {
                            count += var;
                        }
                        if x < width - 1 {
                            timelines
                                .entry(x + 1)
                                .and_modify(|v| {
                                    *v += var;
                                })
                                .or_insert(var);
                        } else {
                            count += var;
                        }
                    }
                    _ => {}
                }
            }
        }
        // end the last line, add with remaining timelines
        count += timelines.values().sum::<u64>();
        count
    }
}
