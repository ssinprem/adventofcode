pub fn get_multipy(round: usize, size: usize) -> Vec<isize> {
    let set = [
        [0_isize].repeat(round),
        [1_isize].repeat(round),
        [0_isize].repeat(round),
        [-1_isize].repeat(round),
    ]
    .concat();

    let mut total = vec![];
    while total.len() < size + 1 {
        total = [total, set.clone()].concat();
    }

    total.into_iter().skip(1).take(size).collect()
}

pub fn parse(file: String) -> Vec<isize> {
    file.chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>()
}

pub fn calculate(num: Vec<isize>) -> Vec<isize> {
    let size = num.len();
    let temp = num.clone();
    (1..=size)
        .map(|r| {
            temp.iter()
                .zip(get_multipy(r, size))
                .map(|(n, m)| n * m)
                .sum::<isize>()
                .abs()
                % 10
        })
        .collect::<Vec<isize>>()
}
pub mod part1 {
    use crate::{calculate, parse};

    pub fn solve(file: String) -> u64 {
        let mut num = parse(file);
        for _ in 0..100 {
            num = calculate(num)
        }

        num.into_iter()
            .take(8)
            .reduce(|acc, n| acc * 10 + n)
            .unwrap() as u64
    }
}

pub mod part2 {
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let mut num = parse(file);
        let offset = num
            .clone()
            .into_iter()
            .take(7)
            .reduce(|acc, n| acc * 10 + n)
            .unwrap();
        num = num.repeat(10_000);
        num = num.into_iter().skip(offset as usize).collect();
        for _ in 0..100 {
            for i in (0..num.len() - 1).rev() {
                num[i] = (num[i] + num[i + 1]) % 10;
            }
        }

        num.into_iter()
            .take(8)
            .reduce(|acc, n| acc * 10 + n)
            .unwrap() as u64
    }
}
