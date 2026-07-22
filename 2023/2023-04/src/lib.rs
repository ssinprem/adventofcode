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

    use super::*;

    pub fn solve(file: String) -> u64 {
        let cards = parse(file);
        let mut sort_cards: Vec<_> = cards.iter().collect();
        sort_cards.sort_by_key(|(id, _)| *id);
        let mut num_cards = vec![1; sort_cards.len()];

        for (no, (_id, (win_nums, nums))) in sort_cards.iter().enumerate() {
            let amount = num_cards[no];
            let wins = win_nums
                .iter()
                .filter(|win_num| nums.contains(*win_num))
                .count();
            for next in (no + 1)..(no + 1 + wins) {
                if next < num_cards.len() {
                    num_cards[next] += amount;
                }
            }
        }

        num_cards.into_iter().sum::<usize>() as u64
    }
}
