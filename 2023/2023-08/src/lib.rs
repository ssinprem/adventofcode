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
    fn gcd(mut a: u64,mut b: u64) -> u64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    fn lcm(a: u64, b: u64) -> u64 {
        (a*b) / gcd(a,b)
    }

    fn lcms(list : &[u64]) -> u64 {
        list.iter().fold(list[0],|acc, n| lcm(acc,*n))
    }

    use crate::parse;

    pub fn solve(file: String) -> u64 {
        let (steps, nodes) = parse(file);
        let starts: Vec<String> = nodes
            .iter()
            .filter_map(|(node, _next)| {
                if node.ends_with("A") {
                    Some(node.to_string())
                } else {
                    None
                }
            })
            .collect();
        println!("{starts:?}");
        
        let counts : Vec<u64> = starts.iter().map(|start| {
            let mut index = 0;
            let mut count = 0;
            let mut cur = start.to_string();
            while !cur.ends_with("Z") {
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
        }).collect();
        
        println!("{counts:?}");

        lcms(counts.as_slice())
    }
}
