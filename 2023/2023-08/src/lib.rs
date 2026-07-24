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

    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (steps, nodes) = parse(file);
        let mut cur = "AAA".to_string();

        let mut index = 0;
        let mut count = 0;
        while cur != "ZZZ" {
            let step = steps.chars().nth(index).unwrap();
            let node = nodes.get(&cur).unwrap();
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
    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (steps, nodes) = parse(file);
        let mut curs: Vec<_> = nodes
            .iter()
            .filter_map(|(node, _next)| {
                if node.ends_with("A") {
                    Some(node.to_string())
                } else {
                    None
                }
            })
            .collect();
        println!("{curs:?}");
        let mut index = 0;
        let mut count = 0;
        while !curs.iter().all(|cur| cur.ends_with("Z")) {
            let step = steps.chars().nth(index).unwrap();
            for cur in curs.iter_mut() {
                let node = nodes.get(cur).unwrap();
                match step {
                    'L' => *cur = node.0.to_string(),
                    'R' => *cur = node.1.to_string(),
                    _ => unreachable!(),
                }
            }
            count += 1;
            index = (index + 1) % steps.len();
            let countz = curs.iter().filter(|cur| cur.ends_with("Z")).count();
            if countz > 1 {
                println!("{count} {countz} {curs:?}");
            }
        }
        count
    }
}
