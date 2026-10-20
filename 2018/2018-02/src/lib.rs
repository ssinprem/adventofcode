pub fn parse(file: String) -> Vec<String> {
    file.lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

pub mod part1 {
    use super::*;
    use std::collections::HashMap;

    pub fn solve(file: String) -> usize {
        let list = parse(file);

        let mut twice = 0;
        let mut triple = 0;

        list.iter().for_each(|line| {
            let mut count = HashMap::new();
            line.chars().for_each(|char| {
                count.entry(char).and_modify(|c| *c += 1).or_insert(1);
            });

            if count.values().any(|v| *v == 2) {
                twice += 1;
            }
            if count.values().any(|v| *v == 3) {
                triple += 1;
            }
        });

        twice * triple
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
