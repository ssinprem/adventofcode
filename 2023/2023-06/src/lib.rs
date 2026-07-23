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
    use super::*;
    pub fn solve(file: String) -> u64 {
        let races = parse(file);
        let race = races.iter().fold((0, 0), |acc, (time, distance)| {
            (
                (acc.0.to_string() + &time.to_string())
                    .parse::<u64>()
                    .unwrap(),
                (acc.1.to_string() + &distance.to_string())
                    .parse::<u64>()
                    .unwrap(),
            )
        });
        println!("{race:?}");
        let time = race.0;
        let distance = race.1;
        (1..time)
            .filter(|hold| hold * (time - hold) > distance)
            .count() as u64
    }
}
