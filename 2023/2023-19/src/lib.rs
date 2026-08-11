use regex::*;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Flow {
    CondNode(char, bool, u64, String),
    CondBool(char, bool, u64, bool),
    Node(String),
    Decide(bool),
    Error,
}

#[allow(clippy::type_complexity)]
pub fn parse(file: String) -> (HashMap<String, Vec<Flow>>, Vec<HashMap<char, u64>>) {
    let mut hm = HashMap::new();
    let mut nums = Vec::new();

    let (str_conds, str_nums) = file.split_once("\n\n").unwrap();
    let str_conds = str_conds.to_string();
    let str_nums = str_nums.to_string();

    str_conds.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        let regex_node = Regex::new(r"(\w+)\{(.*)\}").unwrap();

        let iter = regex_node.captures(line).unwrap();
        let node = iter.get(1).unwrap().as_str().to_string();
        let str_cond = iter.get(2).unwrap().as_str().to_string();

        let regex_grp = Regex::new(
            r"(?:([xmas])([<>])(\d+):([a-z]+))|(?:([xmas])([<>])(\d+):([A|R]))|([a-z]+)|(A|R)",
        )
        .unwrap();

        hm.insert(
            node,
            regex_grp
                .captures_iter(&str_cond)
                .map(|cap| {
                    if let Some(char) = cap.get(1)
                        && let Some(cond) = cap.get(2)
                        && let Some(val) = cap.get(3)
                        && let Some(des) = cap.get(4)
                    {
                        Flow::CondNode(
                            char.as_str().chars().next().unwrap(),
                            cond.as_str() == ">",
                            val.as_str().parse::<u64>().unwrap(),
                            des.as_str().to_string(),
                        )
                    } else if let Some(char) = cap.get(5)
                        && let Some(cond) = cap.get(6)
                        && let Some(val) = cap.get(7)
                        && let Some(dec) = cap.get(8)
                    {
                        Flow::CondBool(
                            char.as_str().chars().next().unwrap(),
                            cond.as_str() == ">",
                            val.as_str().parse::<u64>().unwrap(),
                            dec.as_str() == "A",
                        )
                    } else if let Some(str) = cap.get(9) {
                        Flow::Node(str.as_str().to_string())
                    } else if let Some(char) = cap.get(10) {
                        if char.as_str() == "A" {
                            Flow::Decide(true)
                        } else if char.as_str() == "R" {
                            Flow::Decide(false)
                        } else {
                            Flow::Error
                        }
                    } else {
                        Flow::Error
                    }
                })
                .collect(),
        );
    });

    str_nums.lines().for_each(|line| {
        let regex_nums = Regex::new(r"([xmas])=(\d+)").unwrap();
        nums.push(
            regex_nums
                .captures_iter(line)
                .map(|cap| {
                    let char = cap.get(1).unwrap().as_str().chars().next().unwrap();
                    let num = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    (char, num)
                })
                .collect::<HashMap<_, _>>(),
        );
    });

    (hm, nums)
}

pub fn result(cond: &HashMap<String, Vec<Flow>>, num: &HashMap<char, u64>) -> bool {
    let mut curr_node = "in".to_string();
    loop {
        let node_cond = cond.get(&curr_node).unwrap();
        for cond in node_cond {
            match cond {
                Flow::Error => {
                    unreachable!()
                }
                Flow::Decide(decide) => return *decide,
                Flow::Node(node) => {
                    curr_node = node.to_string();
                    break;
                }
                Flow::CondBool(char, greater, val, decide) => {
                    let cval = num.get(char).unwrap();
                    let result = if *greater { cval > val } else { cval < val };

                    if !result {
                        continue;
                    }
                    return *decide
                }
                Flow::CondNode(char, greater, val, node) => {
                    let cval = num.get(char).unwrap();
                    let result = if *greater { cval > val } else { cval < val };

                    if !result {
                        continue;
                    }
                    curr_node = node.to_string();
                    break;
                }
            }
        }
    }
}

pub mod part1 {

    use super::*;

    pub fn solve(file: String) -> u64 {
        let (cond, nums) = parse(file);
        nums.par_iter()
            .map(|num| {
                if result(&cond, num) {
                    num.values().sum()
                } else {
                    0
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
