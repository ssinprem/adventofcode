pub mod part1 {
    use std::collections::HashMap;

    pub fn solve(file: String) -> u64 {
        let mut reg: HashMap<String, i64> = HashMap::new();
        let mut pc = 0;
        let lines = file
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let mut last_play = None;

        while pc < lines.len() {
            let line = lines[pc].to_string();
            let split = line
                .split_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<String>>();

            let (op1, val1) = if let Ok(val) = split[1].parse::<i64>() {
                (None, Some(val))
            } else if let Some(val) = reg.get(&split[1]) {
                (Some(split[1].to_string()), Some(*val))
            } else {
                (Some(split[1].to_string()), Some(0))
            };
            let (_op2, val2) = if split.len() < 3 {
                (None, None)
            } else if let Ok(val) = split[2].parse::<i64>() {
                (None, Some(val))
            } else if let Some(val) = reg.get(&split[2]) {
                (Some(split[2].to_string()), Some(*val))
            } else {
                (Some(split[2].to_string()), Some(0))
            };

            match split[0].as_str() {
                "snd" => {
                    if let Some(val1) = val1 {
                        last_play = Some(val1);
                    }
                    pc += 1;
                }
                "set" => {
                    if let Some(r) = op1
                        && let Some(val2) = val2
                    {
                        reg.insert(r, val2);
                    }
                    pc += 1;
                }
                "add" => {
                    if let Some(r) = op1
                        && let Some(val1) = val1
                        && let Some(val2) = val2
                    {
                        reg.insert(r, val1 + val2);
                    }
                    pc += 1;
                }
                "mul" => {
                    if let Some(r) = op1
                        && let Some(val1) = val1
                        && let Some(val2) = val2
                    {
                        reg.insert(r, val1 * val2);
                    }
                    pc += 1;
                }
                "mod" => {
                    if let Some(r) = op1
                        && let Some(val1) = val1
                        && let Some(val2) = val2
                    {
                        reg.insert(r, val1 % val2);
                    }
                    pc += 1;
                }
                "rcv" => {
                    if val1 != Some(0)
                        && let Some(r) = op1
                        && r == "a"
                        && let Some(lp) = last_play
                    {
                        return lp as u64;
                    }
                    pc += 1;
                }
                "jgz" => {
                    if let Some(val1) = val1
                        && let Some(val2) = val2
                    {
                        if val1 > 0 {
                            pc = (pc as i64 + val2) as usize;
                        } else {
                            pc += 1;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        0
    }
}

pub mod part2 {
    use std::{collections::{HashMap, VecDeque}};

    pub fn solve(file: String) -> usize {
        let lines = file
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let mut stack = VecDeque::new();
        let mut buffer = [VecDeque::new(), VecDeque::new()];
        let mut output = [0, 0];

        stack.push_back((0, 0, HashMap::from([("p".to_string(),0)])));
        stack.push_back((1, 0, HashMap::from([("p".to_string(),1)])));

        while let Some((id ,mut pc, mut reg)) = stack.pop_front() {
            let line = lines[pc].to_string();
            
            let split = line
                .split_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<String>>();

            let (op1, val1) = if let Ok(val) = split[1].parse::<i64>() {
                (None, Some(val))
            } else if let Some(val) = reg.get(&split[1]) {
                (Some(split[1].to_string()), Some(*val))
            } else {
                (Some(split[1].to_string()), Some(0))
            };
            let (_op2, val2) = if split.len() < 3 {
                (None, None)
            } else if let Ok(val) = split[2].parse::<i64>() {
                (None, Some(val))
            } else if let Some(val) = reg.get(&split[2]) {
                (Some(split[2].to_string()), Some(*val))
            } else {
                (Some(split[2].to_string()), Some(0))
            };
            match split[0].as_str() {
                "snd" => {
                    if let Some(val1) = val1 {
                        buffer[1-id].push_back(val1);
                        output[id] += 1;
                    }
                    pc += 1;
                }
                "set" => {
                    if let Some(r) = op1
                        && let Some(val2) = val2
                    {
                        reg.insert(r, val2);
                    }
                    pc += 1;
                }
                "add" => {
                    if let Some(r) = op1
                        && let Some(val1) = val1
                        && let Some(val2) = val2
                    {
                        reg.insert(r, val1 + val2);
                    }
                    pc += 1;
                }
                "mul" => {
                    if let Some(r) = op1
                        && let Some(val1) = val1
                        && let Some(val2) = val2
                    {
                        reg.insert(r, val1 * val2);
                    }
                    pc += 1;
                }
                "mod" => {
                    if let Some(r) = op1
                        && let Some(val1) = val1
                        && let Some(val2) = val2
                    {
                        reg.insert(r, val1 % val2);
                    }
                    pc += 1;
                }
                "rcv" => {
                    if let Some(last) = buffer[id].pop_front()
                    && let Some(r) = op1
                    {
                        reg.insert(r, last);
                        pc += 1;
                    } else {
                        if stack.iter().all(|(id, spc, _reg)| {
                            lines[*spc].starts_with("rcv")
                            && buffer[*id].len() == 0
                        })
                        {
                            break;
                        } else {
                            stack.push_back((id, pc, reg));
                            continue;
                        }
                    }
                }
                "jgz" => {
                    if let Some(val1) = val1
                        && let Some(val2) = val2
                    {
                        if val1 > 0 {
                            pc = (pc as i64 + val2) as usize;
                        } else {
                            pc += 1;
                        }
                    }
                }
                _ => unreachable!(),
            }
            stack.push_front((id, pc, reg));
        }
        output[1]
    }
}
