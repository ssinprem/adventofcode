pub fn parse(file: String) -> Vec<String> {
    file.split_whitespace()
        .map(|str| str.to_string())
        .collect::<Vec<String>>()
}

pub fn process(set: Vec<String>) -> Vec<String> {
    set
        .iter()
        .flat_map(|str| match (str, str.len()) {
            (str, _) if str == "0" => vec!["1".to_string()],
            (str, len) if len % 2 == 0 => {
                let res = str.split_at(len / 2);
                [res.0, res.1]
                    .iter()
                    .map(|num| num.parse::<u64>().expect("cannot parse").to_string())
                    .collect()
            }
            (str, _) => {
                let mut num = str.parse::<u64>().expect("parse digit");
                num *= 2024;
                vec![num.to_string()]
            }
        })
        .collect()
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let mut set = parse(file);

        (0..25).for_each(|_| {
            set = process(set.clone());
        });

        set.len() as u64
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let mut set = parse(file);

        (0..75).for_each(|_| {
            set = process(set.clone());
        });

        set.len() as u64
    }
}
