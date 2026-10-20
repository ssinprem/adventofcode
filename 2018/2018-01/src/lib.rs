pub fn parse(file: String) -> Vec<i64> {
    file.split_whitespace()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub mod part1 {
    use crate::parse;

    pub fn solve(file: String) -> i64 {
        let mut value = 0;
        let fs = parse(file);

        fs.iter().for_each(|v| {
            value += *v;
        });

        value
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
