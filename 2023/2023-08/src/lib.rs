use regex::Regex;
use std::collections::HashMap;

pub fn parse(file: String) -> (String, HashMap<String, (String, String)>) {
    let steps = file.lines().find(|line| !line.is_empty()).unwrap();

    let regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    (
        steps.to_string(),
        regex
            .captures_iter(&file)
            .map(|cap| {
                (
                    cap.get(1).unwrap().as_str().to_string(),
                    (
                        cap.get(2).unwrap().as_str().to_string(),
                        cap.get(3).unwrap().as_str().to_string(),
                    ),
                )
            })
            .collect(),
    )
}

pub mod part1 {

    use std::println;

    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (steps, nodes) = parse(file);
        println!("{steps}   {nodes:?}");
        let mut cur = "AAA".to_string();

        let mut index = 0;
        let mut count = 0;
        while cur != "ZZZ" {
            let step = steps.chars().nth(index).unwrap();
            let node = nodes.get(&cur).unwrap();
            println!("{step} {cur} {node:?}");
            match step {
                'L' => cur = node.0.to_string(),
                'R' => cur = node.1.to_string(),
                _ => unreachable!(),
            }
            count += 1;
            index = (index + 1) % steps.len();
        }
        count
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
