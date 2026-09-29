use crate::CMD::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CMD {
    Out(String),
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
                "out" => Out(arg[1].to_string()),
                "inc" => Inc(arg[1].to_string()),
                "dec" => Dec(arg[1].to_string()),
                _ => unimplemented!(),
            }
        })
        .collect()
}

fn execute(regs: &mut HashMap<String, isize>, cmds: &[CMD], max_out: usize) -> Vec<isize> {
        let mut out = vec![];
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
                Out(a) => {
                    let value = if let Ok(v) = a.parse::<isize>() {
                        v
                    } else {
                        *regs.get(&a.to_string()).unwrap()
                    };
                    out.push(value);
                    if out.len() > max_out {
                        break;
                    }
                    pc += 1;
                }
            }
        }
        out
    }

pub mod part1 {
    use std::collections::HashMap;

use crate::{execute, parse};

    pub fn solve(file: String) -> u64 {
        let cmds = parse(file);

        for a in 1.. {
            let mut regs = HashMap::new();
            regs.insert("a".to_string(), a);
            let mut out = execute(&mut regs, &cmds, 10);
            println!("{a:5} {out:?}");
            out.truncate(10);
            if out == [0,1,0,1,0,1,0,1,0,1] {
                return a as u64
            }
        }
            
        // let mut regs = HashMap::new();
        // regs.insert("a".to_string(), 198);
        // let out = execute(&mut regs, &cmds, 2000);
        // println!("{out:?}");

        0
    }
}

pub mod part2 {
    pub fn solve(_file: String) -> u64 {
        0
    }
}
