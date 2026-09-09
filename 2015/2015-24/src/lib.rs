use itertools::Itertools;

pub fn parse(file: String) -> Vec<u64> {
    file.lines().filter_map(|line| {
        if let Ok(n) = line.parse::<u64>() {
            Some(n)
        } else {
            None
        }
    }).collect()
}

fn find_best_qe(packages: &[u64], num_groups: u64) -> u64 {
    let total_weight: u64 = packages.iter().sum();
    let target_weight = total_weight / num_groups;

    for k in 1..packages.len() {
        if let Some(qe) = packages
            .iter()
            .combinations(k)
            .filter(|c| c.iter().copied().sum::<u64>() == target_weight)
            .map(|c| c.iter().copied().product::<u64>())
            .min() {
            return qe;
        }
    }
    unreachable!()
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let n_list = parse(file);
        find_best_qe(&n_list, 3)
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let n_list = parse(file);
        find_best_qe(&n_list, 4)
    }
}
