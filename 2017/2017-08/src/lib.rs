use regex::Regex;
use std::collections::HashMap;

pub fn solve(file: String) -> (HashMap<String, i32>, HashMap<String, i32>) {
    let mut max = HashMap::new();
    let mut reg = HashMap::new();
    let regex = Regex::new(r"(\w+) (inc|dec) (-{0,1}[0-9]+) if (\w+) ([<>=!]+) (-{0,1}[0-9]+)")
        .unwrap();

    file.lines().for_each(|line| {
        if let Some(matched) = regex.captures(line)
            && let Some(reg_name) = matched.get(1)
            && let reg_name = reg_name.as_str().to_string()
            && let Some(opcode) = matched.get(2)
            && let opcode = opcode.as_str().to_string()
            && let Some(oprand) = matched.get(3)
            && let Ok(oprand) = oprand.as_str().parse::<i32>()
            && let Some(cond_reg) = matched.get(4)
            && let cond_reg = cond_reg.as_str().to_string()
            && let Some(cond_opcode) = matched.get(5)
            && let cond_opcode = cond_opcode.as_str().to_string()
            && let Some(cond_value) = matched.get(6)
            && let Ok(cond_value) = cond_value.as_str().parse::<i32>()
        {
            let cond_reg = if let Some(reg_value) = reg.get_mut(&cond_reg) {
                reg_value
            } else {
                reg.insert(cond_reg.to_string(), 0);
                reg.get_mut(&cond_reg).unwrap()
            };

            if match cond_opcode.as_str() {
                ">" => *cond_reg > cond_value,
                "<" => *cond_reg < cond_value,
                ">=" => *cond_reg >= cond_value,
                "<=" => *cond_reg <= cond_value,
                "==" => *cond_reg == cond_value,
                "!=" => *cond_reg != cond_value,
                _ => unreachable!(),
            } {
                let reg_value = if let Some(reg_value) = reg.get_mut(&reg_name) {
                    reg_value
                } else {
                    reg.insert(reg_name.to_string(), 0);
                    reg.get_mut(&reg_name).unwrap()
                };
                match opcode.as_str() {
                    "inc" => *reg_value += oprand,
                    "dec" => *reg_value -= oprand,
                    _ => unreachable!(),
                }
                max.entry(reg_name).and_modify(|v| {
                    if *v < *reg_value {
                        *v = *reg_value
                    }
                }).or_insert(*reg_value);
            }
        } else {
            unreachable!()
        }
    });
    (reg,max)
}

pub mod part1 {
    pub fn solve(file: String) -> i32 {
        let (reg,_max) = crate::solve(file);
        *reg.values().max().unwrap()
    }
}

pub mod part2 {
    pub fn solve(file: String) -> i32 {
        let (_reg, max) = crate::solve(file);
        *max.values().max().unwrap()
    }
}
