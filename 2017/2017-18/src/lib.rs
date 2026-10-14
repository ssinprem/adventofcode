pub mod part1 {
    use std::collections::HashMap;

    fn _pause() {
        use std::io;
        use std::io::prelude::*;
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        write!(stdout, "Press any key to continue...").unwrap();
        stdout.flush().unwrap();

        let _ = stdin.read(&mut [0u8]).unwrap();
    }

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
    pub fn solve(_file: String) -> u64 {
        0
    }
}
