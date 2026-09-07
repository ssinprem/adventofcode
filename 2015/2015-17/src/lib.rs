use std::collections::VecDeque;

pub fn parse(file: String) -> Vec<u64> {
    file.lines()
        .filter_map(|line| line.parse::<u64>().ok())
        .collect()
}

pub fn find_fit_possible(containers: Vec<u64>, eggnog: u64) -> Vec<Vec<u64>> {
    let mut resolves = Vec::new();
    let mut stack: VecDeque<Vec<u64>> = VecDeque::new();

    for con in containers {
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
    resolves
}
pub mod part1 {
    use crate::{find_fit_possible, parse};
    use std::cmp::Reverse;

    pub fn solve(file: String, eggnog: u64) -> u64 {
        let mut containers = parse(file);
        containers.sort_by_key(|n| Reverse(*n));

        let resolves = find_fit_possible(containers, eggnog);
        resolves.len() as u64
    }
}

pub mod part2 {
    use crate::{find_fit_possible, parse};
    use std::cmp::Reverse;

    pub fn solve(file: String, eggnog: u64) -> u64 {
        let mut containers = parse(file);
        containers.sort_by_key(|n| Reverse(*n));

        let resolves = find_fit_possible(containers, eggnog);

        let min_con = resolves.iter().map(|con| con.len()).min().unwrap();
        resolves.iter().filter(|con| con.len() == min_con).count() as u64
    }
}
