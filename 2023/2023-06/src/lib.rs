pub fn parse(file: String) -> Vec<(u32, u32)> {
    let tline = file.lines().find(|line| line.starts_with("Time:")).unwrap();
    let dline = file
        .lines()
        .find(|line| line.starts_with("Distance:"))
        .unwrap();

    tline
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .zip(
            dline
                .strip_prefix("Distance:")
                .unwrap()
                .split_whitespace()
                .map(|str| str.parse().unwrap()),
        )
        .collect()
}

pub mod part1 {
    use std::println;

    use super::*;
    pub fn solve(file: String) -> u64 {
        let races = parse(file);
        println!("{races:?}");
        races
            .iter()
            .map(|(time, distance)| {
                (1..*time)
                    .filter(|hold| hold * (time - hold) > *distance)
                    .count() as u32
            })
            .product::<u32>()
            .into()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
