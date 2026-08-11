use rayon::prelude::*;
use regex::*;
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
                    return *decide;
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
    use std::{collections::HashMap, println};

    use crate::{Flow, parse};

    type Ranges = HashMap<char, (u64, u64)>;

    fn count_accepted_combinations(
        workflow_name: &str,
        mut ranges: Ranges,
        workflows: &HashMap<String, Vec<Flow>>,
    ) -> u64 {
        if workflow_name == "A" {
            return ranges.values().map(|(min, max)| max - min + 1).product();
        }
        if workflow_name == "R" {
            return 0;
        }

        let workflow = workflows.get(workflow_name).unwrap();
        let mut total = 0;
        println!("{ranges:?}");
        for flow in workflow {
            println!(" {workflow_name} {flow:?}");
            match flow {
                Flow::CondNode(c, gt, val, next_node) => {
                    let (min, max) = ranges.get(c).unwrap();
                    let (true_range, false_range) = if *gt {
                        ((val + 1, *max), (*min, *val))
                    } else {
                        ((*min, val - 1), (*val, *max))
                    };

                    if true_range.0 <= true_range.1 {
                        let mut next_ranges = ranges.clone();
                        next_ranges.insert(*c, true_range);
                        total += count_accepted_combinations(next_node, next_ranges, workflows);
                    }
                    if false_range.0 <= false_range.1 {
                        ranges.insert(*c, false_range);
                    } else {
                        // No more ranges to check for this path
                        return total;
                    }
                }
                Flow::CondBool(c, gt, val, next_bool) => {
                    let (min, max) = ranges.get(c).unwrap();
                    let (true_range, false_range) = if *gt {
                        ((val + 1, *max), (*min, *val))
                    } else {
                        ((*min, val - 1), (*val, *max))
                    };

                    if true_range.0 <= true_range.1 {
                        let mut next_ranges = ranges.clone();
                        next_ranges.insert(*c, true_range);
                        total += count_accepted_combinations(
                            if *next_bool { "A" } else { "R" },
                            next_ranges,
                            workflows,
                        );
                    }
                    if false_range.0 <= false_range.1 {
                        ranges.insert(*c, false_range);
                    } else {
                        // No more ranges to check for this path
                        return total;
                    }
                }
                Flow::Node(next_node) => {
                    total += count_accepted_combinations(next_node, ranges.clone(), workflows);
                    // This is a terminal rule for this path, so we can stop processing this workflow.
                    return total;
                }
                Flow::Decide(decide) => {
                    total += count_accepted_combinations(
                        if *decide { "A" } else { "R" },
                        ranges.clone(),
                        workflows,
                    );
                    // This is a terminal rule for this path, so we can stop processing this workflow.
                    return total;
                }
                Flow::Error => unreachable!(),
            }
        }
        total
    }

    pub fn solve(file: String) -> u64 {
        let (workflows, _) = parse(file);
        let mut ranges: Ranges = HashMap::new();
        ranges.insert('x', (1, 4000));
        ranges.insert('m', (1, 4000));
        ranges.insert('a', (1, 4000));
        ranges.insert('s', (1, 4000));

        count_accepted_combinations("in", ranges, &workflows)
    }
}
