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
    use crate::parse;

    pub fn solve(file: String) -> i64 {
        let mut value = 0;
        let fs = parse(file);
        let mut history = Vec::new();
        history.push(0);
        loop {
            for v in fs.iter() {
                value += *v;
                if history.contains(&value) {
                    return value;
                }
                history.push(value);
            }
        }
    }
}
