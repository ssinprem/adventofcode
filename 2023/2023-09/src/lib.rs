pub fn parse(file: String) -> Vec<Vec<i64>> {
    file.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|str| str.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn get_next(event: &[i64]) -> i64 {
    let diff: Vec<i64> = event.windows(2).map(|pair| pair[1] - pair[0]).collect();
    if diff.iter().all(|d| *d == 0) {
        event[0]
    } else {
        event.last().unwrap() + get_next(&diff)
    }
}

pub fn get_prv(event: &[i64]) -> i64 {
    let diff: Vec<i64> = event.windows(2).map(|pair| pair[1] - pair[0]).collect();
    if diff.iter().all(|d| *d == 0) {
        event[0]
    } else {
        event.first().unwrap() - get_prv(&diff)
    }
}

pub mod part1 {
    use crate::{get_next, parse};
    pub fn solve(file: String) -> u64 {
        let events = parse(file);

        events
            .into_iter()
            .inspect(|evt| println!("{evt:?}"))
            .map(|event| get_next(&event))
            .inspect(|n| println!("{n}"))
            .sum::<i64>() as u64
    }
}

pub mod part2 {
    use crate::{get_prv, parse};
    pub fn solve(file: String) -> u64 {
        let events = parse(file);

        events
            .into_iter()
            .inspect(|evt| println!("{evt:?}"))
            .map(|event| get_prv(&event))
            .inspect(|n| println!("{n}"))
            .sum::<i64>() as u64
    }
}
