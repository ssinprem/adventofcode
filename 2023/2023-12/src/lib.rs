pub fn get_possible(record: String, list: Vec<usize>) -> u64 {
    println!("{record}");
    let qnum = record.chars().filter(|char| char == &'?').count();

    (0..2_i32.pow(qnum as u32))
        .filter(|n| {
            let mut rec = record.to_string();
            for i in 0..qnum {
                if (n >> i) % 2 == 0 {
                    rec = rec.replacen("?", ".", 1);
                } else {
                    rec = rec.replacen("?", "#", 1);
                }
            }
            let cap: Vec<_> = rec
                .replace(".", " ")
                .split_whitespace()
                .map(|str| str.len())
                .collect();
            let matched = cap == list;
            if matched {
                println!("{rec} {cap:?} {list:?}");
            }
            matched
        })
        .count() as u64
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        file.lines()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    let (record, list) = line.split_once(" ").unwrap();
                    let list = list
                        .split(",")
                        .map(|str| str.parse::<usize>().unwrap())
                        .collect();
                    Some(get_possible(record.to_string(), list))
                }
            })
            .sum::<u64>()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
