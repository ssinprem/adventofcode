use Node::*;
use regex::Regex;

use crate::Inst::Compare;
#[derive(Clone, Debug)]
pub enum Node {
    Output(usize),
    Robot(usize),
}

#[derive(Clone, Debug)]
pub enum Inst {
    Give(Node, u32),
    Compare(Node, Node, Node),
}

pub fn parse(file: String) -> Vec<Inst> {
    let regex = Regex::new(
        r"(?:value (\d+) goes to bot (\d+))|(?:bot (\d+) gives low to (output|bot) (\d+) and high to (bot|output) (\d+))"
    ).unwrap();

    file.lines()
        .map(|line| {
            if let Some(caps) = regex.captures(line) {
                if let Some(robot) = caps.get(2)
                    && let Ok(robot_id) = robot.as_str().parse::<usize>()
                    && let Some(chip) = caps.get(1)
                    && let Ok(chip_value) = chip.as_str().parse::<u32>()
                {
                    Inst::Give(Robot(robot_id), chip_value)
                } else if let Some(robot) = caps.get(3)
                    && let Ok(robot_id) = robot.as_str().parse::<usize>()
                    && let Some(type1) = caps.get(4)
                    && let Some(id1) = caps.get(5)
                    && let Ok(id1) = id1.as_str().parse::<usize>()
                    && let Some(type2) = caps.get(6)
                    && let Some(id2) = caps.get(7)
                    && let Ok(id2) = id2.as_str().parse::<usize>()
                {
                    let low_des = if type1.as_str() == "bot" {
                        Robot(id1)
                    } else if type1.as_str() == "output" {
                        Output(id1)
                    } else {
                        unreachable!()
                    };
                    let high_des = if type2.as_str() == "bot" {
                        Robot(id2)
                    } else if type2.as_str() == "output" {
                        Output(id2)
                    } else {
                        unreachable!()
                    };

                    Compare(Robot(robot_id), low_des, high_des)
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        })
        .collect()
}

pub mod part1 {
    use crate::{
        Inst::{Compare, Give},
        Node::Robot,
        parse,
    };
    use std::collections::VecDeque;

    pub fn solve(file: String, target_chips: &[u32]) -> Option<u32> {
        let mut result = None;
        let mut steps = VecDeque::from(parse(file));
        let max_bot = steps
            .iter()
            .map(|step| match step {
                Give(Robot(id), _) => *id,
                Compare(Robot(id), _, _) => *id,
                _ => 0,
            })
            .max()
            .unwrap();

        let mut robots: Vec<Vec<u32>> = vec![vec![]; max_bot + 1];
        while let Some(step) = steps.pop_front() {
            match step.clone() {
                Give(Robot(id), value) => {
                    println!("Robot({id:3}) take  {value}");
                    robots[id].push(value);
                }
                Compare(Robot(id), low_dest, high_dest) => {
                    if robots[id].len() == 2 {
                        robots[id].sort();
                        println!(
                            "Robot({id:3})  {:3?}  to {low_dest:3?}  {high_dest:3?}",
                            robots[id]
                        );
                        let high = robots[id].pop().unwrap();
                        let low = robots[id].pop().unwrap();

                        if target_chips.contains(&high) && target_chips.contains(&low) {
                            result = Some(id as u32);
                        }

                        if let Robot(dest_id) = low_dest {
                            robots[dest_id].push(low)
                        }
                        if let Robot(dest_id) = high_dest {
                            robots[dest_id].push(high)
                        }
                    } else {
                        steps.push_back(step);
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }

        result
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
