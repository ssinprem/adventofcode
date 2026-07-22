use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn parse(file: String) -> HashMap<u32, (HashSet<u32>, HashSet<u32>)> {
    let regex_grp = Regex::new(r"Card +(\d+): ([0-9 ]+)\| ([0-9 ]+)").unwrap();
    regex_grp
        .captures_iter(&file)
        .map(|cap| {
            let id = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let winnings = cap.get(2).unwrap().as_str();
            let numbers = cap.get(3).unwrap().as_str();
            (
                id,
                (
                    winnings
                        .split_whitespace()
                        .flat_map(|s| s.parse())
                        .collect(),
                    numbers.split_whitespace().flat_map(|s| s.parse()).collect(),
                ),
            )
        })
        .collect()
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let cards = parse(file);
        cards
            .iter()
            .map(|(_id, (win_nums, nums))| {
                let wins = win_nums
                    .iter()
                    .filter(|win_num| nums.contains(*win_num))
                    .count();
                if wins > 0 { 1 << (wins - 1) } else { 0 }
            })
            .sum()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
