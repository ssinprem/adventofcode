use crate::CMD::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CMD {
    Toggle(String),
    Inc(String),
    Dec(String),
    Copy(String, String),
    JumpNonZero(String, String),
}

pub fn parse(file: String) -> Vec<CMD> {
    file.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let arg = line
                .split_whitespace()
                .map(|str| str.to_string())
                .collect::<Vec<String>>();
            match arg[0].as_str() {
                "cpy" => Copy(arg[1].to_string(), arg[2].to_string()),
                "jnz" => JumpNonZero(arg[1].to_string(), arg[2].to_string()),
                "tgl" => Toggle(arg[1].to_string()),
                "inc" => Inc(arg[1].to_string()),
                "dec" => Dec(arg[1].to_string()),
                _ => unimplemented!(),
            }
        })
        .collect()
}

pub fn toggle(cmd: &CMD) -> CMD {
    match cmd {
        Copy(a, b) => JumpNonZero(a.to_string(), b.to_string()),
        JumpNonZero(a, b) => Copy(a.to_string(), b.to_string()),
        Inc(a) => Dec(a.to_string()),
        Dec(a) => Inc(a.to_string()),
        Toggle(a) => Inc(a.to_string()),
    }
}

pub fn execute(regs: &mut HashMap<String, isize>, cmds: &mut [CMD]) {
    let len = cmds.len();
    let mut pc: isize = 0;
    while (pc as usize) < len {
        let cmd = cmds.get(pc as usize).unwrap().clone();
        // println!("{regs:?} #{pc}  {cmd:?}");
        match cmd {
            Copy(a, b) => {
                let value = if let Ok(v) = a.parse::<isize>() {
                    v
                } else {
                    *regs.get(&a.to_string()).unwrap()
                };

                regs.insert(b.to_string(), value);
                pc += 1;
            }
            JumpNonZero(a, b) => {
                let reg = if let Ok(v) = a.parse::<isize>() {
                    v
                } else {
                    *regs.get(&a.to_string()).unwrap()
                };
                let value = if let Ok(v) = b.parse::<isize>() {
                    v
                } else {
                    *regs.get(&b.to_string()).unwrap()
                };

                if reg != 0 {
                    pc += value;
                } else {
                    pc += 1
                }
            }
            Inc(a) => {
                regs.entry(a.to_string()).and_modify(|reg| *reg += 1);
                pc += 1;
            }
            Dec(a) => {
                regs.entry(a.to_string()).and_modify(|reg| *reg -= 1);
                pc += 1;
            }
            Toggle(a) => {
                let value = if let Ok(v) = a.parse::<isize>() {
                    v
                } else {
                    *regs.get(&a.to_string()).unwrap()
                };
                if let Some(cmd) = cmds.get_mut((pc + value) as usize) {
                    *cmd = toggle(cmd);
                }
                pc += 1;
            }
        }
    }
}

pub mod part1 {
    use crate::{execute, parse};
    use std::collections::HashMap;

    pub fn solve(file: String, addition: String) -> isize {
        let mut cmds = parse(addition + file.as_str());
        let mut regs = HashMap::new();
        execute(&mut regs, &mut cmds);
        println!("{:?}", regs);
        regs.get("a").copied().unwrap()
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
