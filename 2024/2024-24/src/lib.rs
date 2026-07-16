use std::collections::HashMap;
use regex::Regex;

enum Logic {
    Value(bool),
    AND(String,String),
    OR(String,String),
    XOR(String,String),
}

pub fn parse(file: String) -> HashMap<String, Logic> {
    file.lines().filter(|line| !line.is_empty())
    .map(|line| {
        let regex = Regex::new(r"(?([a-z0-9]+): ([0-1]))|(?([a-z0-9]+) (OR|AND|XOR) ([a-z0-9]+) -> ([a-z0-9]+))").unwrap();
        if regex.captures_iter(line) {
            
        }
    })
    .collect()
}

pub mod part1 {
    use super::*;

    pub fn solve(file: String) -> u64 {
        let map = parse(file);
        
        0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
