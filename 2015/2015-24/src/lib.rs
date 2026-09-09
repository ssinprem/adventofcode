
pub fn parse(file: String) -> Vec<u64> {
    file.lines().filter_map(|line| {
        if let Ok(n) = line.parse::<u64>() {
            Some(n)
        } else {
            None
        }
    }).collect()
}

pub mod part1 {
use super::*;

    pub fn solve(file: String) -> u64 {
        let n_list = parse(file);
        let weight = n_list.clone().into_iter().sum::<u64>() / 3;

        let mut possible = vec![vec![]];
        for n in n_list.clone() {
            let mut new_possible = vec![];

            while let Some(mut pos) = possible.pop() {
                new_possible.push(pos.clone());
                pos.push(n);
                if pos.clone().into_iter().sum::<u64>() <= weight {
                    new_possible.push(pos);
                }
            }
            possible = new_possible;
            println!("{n} {}", possible.len());
        }

        possible = possible.into_iter().filter(|pos| pos.into_iter().copied().sum::<u64>() == weight).collect();
        // let target = possible.iter().min_by_key(|v| v.into_iter().copied().product::<u64>()).unwrap();
        let target = possible.iter().min_by_key(|v| v.len()).unwrap();
        println!("{target:?}");
        target.into_iter().copied().product()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
