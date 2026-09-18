use regex::Regex;
use std::collections::{VecDeque, HashMap, HashSet};
use itertools::Itertools;

fn parse(file: String) -> Vec<(usize, usize)> {
    let mut name_to_id = HashMap::new();
    let mut next_id = 0;
    let mut pairs = Vec::new();

    let re_gen = Regex::new(r"(\w+) generator").unwrap();
    let re_chip = Regex::new(r"(\w+)-compatible microchip").unwrap();

    for (floor_idx, line) in file.lines().enumerate() {
        for cap in re_gen.captures_iter(line) {
            let name = cap.get(1).unwrap().as_str();
            let id = *name_to_id.entry(name.to_string()).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                pairs.resize(id + 1, (0, 0));
                id
            });
            pairs[id].0 = floor_idx;
        }
        for cap in re_chip.captures_iter(line) {
            let name = cap.get(1).unwrap().as_str();
            let id = *name_to_id.entry(name.to_string()).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                pairs.resize(id + 1, (0, 0));
                id
            });
            pairs[id].1 = floor_idx;
        }
    }
    pairs
}

fn is_safe(pairs: &[(usize, usize)]) -> bool {
    for floor_idx in 0..4 {
        if pairs.iter().any(|(gen_floor, _)| gen_floor==&floor_idx) {
            // if there is a generator, check for unprotected chips
            for &(gen_floor, chip_floor) in pairs {
                // if a chip is on this floor, but its generator is not
                if chip_floor == floor_idx && gen_floor != floor_idx {
                    return false; // Found unprotected chip
                }
            }
        }
    }
    true
}


fn is_finished(pairs: &[(usize, usize)]) -> bool {
    pairs.iter().all(|&(g, c)| g == 3 && c == 3)
}

pub fn moving(initial_pairs: Vec<(usize, usize)>) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let mut initial_pairs = initial_pairs;
    initial_pairs.sort();
    
    // The state is defined by the elevator's floor and the floor of each pair of items.
    // A pair is a (generator, microchip).
    // The item names don't matter, only their pairings. So we can represent the state
    // by sorting the pairs, making the state canonical.
    let initial_state = (0, initial_pairs);

    queue.push_back((0, initial_state.clone()));
    visited.insert(initial_state);

    while let Some((steps, (elevator_floor, pairs))) = queue.pop_front() {
        if is_finished(&pairs) {
            return steps;
        }

        let mut items_on_floor = Vec::new();
        for (id, &(gen_floor, chip_floor)) in pairs.iter().enumerate() {
            if gen_floor == elevator_floor {
                items_on_floor.push((id, 0)); // 0 for generator
            }
            if chip_floor == elevator_floor {
                items_on_floor.push((id, 1)); // 1 for chip
            }
        }

        for num_to_move in 1..=2 {
            if items_on_floor.len() < num_to_move {
                continue;
            }
            for items_to_move in items_on_floor.iter().combinations(num_to_move) {
                for direction in [-1, 1] {
                    let next_floor = elevator_floor as i32 + direction;
                    if !(0..=3).contains(&next_floor) {
                        continue;
                    }

                    let mut next_pairs = pairs.clone();
                    for &&(id, item_type) in &items_to_move {
                        if item_type == 0 { // generator
                            next_pairs[id].0 = next_floor as usize;
                        } else { // chip
                            next_pairs[id].1 = next_floor as usize;
                        }
                    }

                    if is_safe(&next_pairs) {
                        next_pairs.sort();
                        let next_state = (next_floor as usize, next_pairs);
                        if !visited.contains(&next_state) {
                            visited.insert(next_state.clone());
                            queue.push_back((steps + 1, next_state));
                        }
                    }
                }
            }
        }
    }

    unreachable!();
}


pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u32 {
        let initial_pairs = parse(file);
        moving(initial_pairs)
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u32 {
        let mut initial_pairs = parse(file);
        // Add two new pairs, both starting on floor 0.
        initial_pairs.push((0, 0)); // elerium
        initial_pairs.push((0, 0)); // dilithium
        moving(initial_pairs)
    }
}
