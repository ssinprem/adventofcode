pub fn parse(file: String) -> Vec<u64> {
    file.lines()
        .filter_map(|line| line.parse::<u64>().ok())
        .collect()
}

pub mod part1 {
    use crate::parse;
    use std::{cmp::Reverse, collections::VecDeque};

    pub fn solve(file: String, eggnog: u64) -> u64 {
        let mut cons = parse(file);
        cons.sort_by_key(|n| Reverse(*n));

        let mut resolves = Vec::new();
        let mut stack: VecDeque<Vec<u64>> = VecDeque::new();

        for con in cons {
            let old_stack = stack.clone();

            for mut s in old_stack.clone() {
                s.push(con);
                stack.push_back(s);
            }
            stack.push_back(vec![con]);

            let mut temp_stack = stack.clone();
            stack.clear();
            while let Some(temp) = temp_stack.pop_back() {
                let sum = temp.iter().sum::<u64>();
                if sum == eggnog {
                    resolves.push(temp);
                } else if sum < eggnog {
                    stack.push_back(temp);
                }
            }
        }
        resolves.len() as u64
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
