use std::collections::HashMap;

pub fn parse(file: String) -> HashMap<String,u64> {
    let mut set = HashMap::<String,u64>::new();
    file.split_whitespace()
        .for_each(|str| {
            set.entry(str.to_string())
                .and_modify(|c| *c+=1)
                .or_insert(1);
        });
    set
}

pub fn process(set: HashMap<String, u64>) -> HashMap<String, u64> {
    let mut new_set = HashMap::new();
    for (str, count) in set {
        match (str.clone(), str.clone().len()) {
            (str, _) if str == "0" => {
                new_set.entry("1".to_string())
                    .and_modify(|c| *c+=count)
                    .or_insert(count);
            },
            (str, len) if len % 2 == 0 => {
                let res = str.split_at(len / 2);
                [res.0, res.1]
                    .iter()
                    .for_each(|s| {
                        let new_str = 
                            s.to_string().parse::<u64>().expect("cannot parse back")
                            .to_string();
                        new_set.entry(new_str)
                            .and_modify(|c| *c+=count)
                            .or_insert(count);
                    });
            }
            (str, _) => {
                let mut num = str.parse::<u64>().expect("parse digit");
                num *= 2024;
                new_set.entry(num.to_string())
                    .and_modify(|c| *c+=count)
                    .or_insert(count);
            }
        }
    }
    new_set
}

pub mod part1 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let mut set = parse(file);

        (0..25).for_each(|_| {
            set = process(set.clone());
        });

        set.values().sum::<u64>() as u64
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(file: String) -> u64 {
        let mut set = parse(file);

        (0..75).for_each(|_| {
            set = process(set.clone());
        });

        set.values().sum::<u64>() as u64
    }
}
