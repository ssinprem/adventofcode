use regex::Regex;
use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum Item {
    Gen(String),
    Chip(String)
}

pub fn parse(file: String) -> Vec<HashSet<Item>> {
    let regex = Regex::new(r"(?:(\w+) generator)|(?:(\w+)-compatible microchip)").unwrap();
    file.lines().map(|line| {
        regex.captures_iter(line).filter_map(|cap| {
            if let Some(gene) = cap.get(1) {
                Some(Item::Gen(gene.as_str().to_string()))
            } else {
                cap.get(2).map(|chip| Item::Chip(chip.as_str().to_string()))
            }
        }).collect()
    }).collect()
}

pub fn is_safe(floors: &[HashSet<Item>]) -> bool {

    floors.iter().all(|items| {
        let temp = items.clone();

        // If no generate return valid
        if items.clone().iter().all(|item| {
            !matches!(item, Item::Gen(_))
        }) {
            return true;
        }

        ! items.iter().any(|item| {
            if let Item::Chip(chip) = item {
                temp.iter().find(|item| 
                    item == &&Item::Gen(chip.to_string())
                ).is_none()
            } else {
                false
            }
        })
    })
}

pub mod part1 {
    use std::collections::{VecDeque, HashMap, HashSet};
    use super::*;

    pub fn solve(file: String) -> u32 {
        let floors = parse(file);
        fn normalize(floors: &[HashSet<Item>]) -> Vec<Vec<Item>> {
            floors.iter().map(|set| {
                let mut v = set.iter().cloned().collect::<Vec<_>>();
                v.sort();
                v
            }).collect()
        }

        let mut visited: HashMap<(Vec<Vec<Item>>, usize), u32> = HashMap::new();
        let mut queues: VecDeque<(Vec<HashSet<Item>>, usize, u32)> = VecDeque::new();
        queues.push_back((floors.clone(), 0, 0));
        visited.insert((normalize(&floors), 0), 0);
        while let Some((floors, elevetor, steps)) = queues.pop_front() {
            if floors[0..3].iter().all(|items| items.is_empty()) {
                return steps;
            }
            
            let items = floors[elevetor]
                .clone().into_iter().collect::<Vec<_>>();

            let moves : Vec<_> = [
                items.clone().into_iter().combinations(1).collect::<Vec<_>>(),
                items.clone().into_iter().combinations(2).collect::<Vec<_>>()
            ].concat();
            for move_items in moves {
                if elevetor < 3 {
                    let mut new_floors = floors.clone();
                    for item in move_items.clone() {
                        new_floors[elevetor].remove(&item);
                        new_floors[elevetor+1].insert(item);
                    }
                    let key = (normalize(&new_floors), elevetor + 1);
                    if is_safe(&new_floors)
                    {
                        let visit = visited.get(&key);
                        if visit.is_none() || steps+1 < *visit.unwrap() {
                            visited.insert(key.clone(), steps + 1);
                            queues.push_back((new_floors.clone(), elevetor + 1, steps + 1));
                        }
                    }
                }
                if elevetor > 0 {
                    let mut new_floors = floors.clone();
                    for item in move_items.clone() {
                        new_floors[elevetor].remove(&item);
                        new_floors[elevetor-1].insert(item);
                    }
                    let key = (normalize(&new_floors), elevetor - 1);
                    if is_safe(&new_floors)
                    {
                        let visit = visited.get(&key);
                        if visit.is_none() || steps+1 < *visit.unwrap() {
                            visited.insert(key.clone(), steps + 1);
                            queues.push_back((new_floors.clone(), elevetor - 1, steps + 1));
                        }
                    }
                }
            }
        }

        0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
